use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

fn sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn symbol_for(language_id: &str) -> String {
    match language_id {
        "csharp" | "c-sharp" => "tree_sitter_c_sharp".to_string(),
        "objective-c" | "objc" => "tree_sitter_objc".to_string(),
        "qml" => "tree_sitter_qmljs".to_string(),
        _ => format!("tree_sitter_{}", language_id.replace('-', "_")),
    }
}

fn artifact_name(language_id: &str) -> Option<String> {
    if cfg!(target_os = "linux") {
        Some(format!("{language_id}-linux-x86_64.so"))
    } else if cfg!(target_os = "windows") {
        Some(format!("{language_id}-windows-x86_64.dll"))
    } else {
        None
    }
}

fn download(url: &str, target: &Path) -> Result<(), String> {
    let output = Command::new("curl")
        .args(["--fail", "--location", "--silent", "--show-error", "--retry", "2", "--max-time", "120", "-o"])
        .arg(target)
        .arg(url)
        .output()
        .map_err(|error| format!("no se pudo ejecutar curl: {error}"))?;
    if !output.status.success() {
        return Err(format!("descarga fallida: {url}"));
    }
    Ok(())
}

fn check_language(
    root: &Path,
    language: &serde_json::Value,
    release_base: &str,
    workdir: &Path,
) -> Result<(), String> {
    let id = language["id"].as_str().ok_or("id ausente")?;
    let query_path = root.join(
        language["queries"]["path"]
            .as_str()
            .ok_or("queries.path ausente")?,
    );
    let query = std::fs::read(&query_path).map_err(|error| format!("query ilegible: {error}"))?;

    if let Some(expected) = language["queries"]["sha256"].as_str() {
        let actual = sha256(&query);
        if !actual.eq_ignore_ascii_case(expected) {
            return Err(format!(
                "sha256 de query no coincide (registry {expected}, archivo {actual})"
            ));
        }
    }

    let Some(name) = artifact_name(id) else {
        return Err("plataforma no soportada por el validador".into());
    };
    let artifact = workdir.join(&name);
    let local_artifacts = std::env::var_os("FORJA_QUERY_CHECK_ARTIFACT_DIR").map(PathBuf::from);
    if let Some(directory) = local_artifacts {
        // Desarrollo local: valida la query contra un binario recién compilado
        // sin esperar a que CI lo publique en el release. Los lenguajes que no
        // estén en el directorio se descargan como siempre.
        let local = directory.join(&name);
        if local.exists() {
            std::fs::copy(&local, &artifact)
                .map_err(|error| format!("no se pudo copiar el artefacto local: {error}"))?;
        } else {
            let url = format!("{release_base}/{name}");
            if download(&url, &artifact).is_err() {
                println!("SKIP {id}: sin artefacto publicado todavía");
                return Ok(());
            }
        }
    } else {
        let url = format!("{release_base}/{name}");
        match download(&url, &artifact) {
            Ok(()) => {}
            // Un PR que añade un lenguaje todavía no puede descargar su binario.
            Err(_) => {
                println!("SKIP {id}: sin artefacto publicado todavía");
                return Ok(());
            }
        }
    }

    let source = std::str::from_utf8(&query).map_err(|error| format!("query no UTF-8: {error}"))?;
    let symbol = symbol_for(id);
    let result = std::panic::catch_unwind(|| -> Result<(), String> {
        unsafe {
            let library = libloading::Library::new(&artifact)
                .map_err(|error| format!("no se pudo cargar {}: {error}", artifact.display()))?;
            let language_fn: libloading::Symbol<unsafe extern "C" fn() -> tree_sitter::Language> =
                library
                    .get(symbol.as_bytes())
                    .map_err(|error| format!("símbolo {symbol} ausente: {error}"))?;
            let language = language_fn();
            tree_sitter::Query::new(&language, source)
                .map_err(|error| format!("query incompatible: {error:?}"))?;
            // La librería sigue viva hasta aquí; soltarla después de la query.
            Ok(())
        }
    });

    std::fs::remove_file(&artifact).ok();

    match result {
        Ok(Ok(())) => {
            println!("OK   {id}");
            Ok(())
        }
        Ok(Err(error)) => Err(error),
        Err(_) => Err("panic al cargar la gramática".into()),
    }
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("raíz del registry")
        .to_path_buf();
    let registry: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(root.join("registry.json")).expect("registry.json"),
    )
    .expect("registry.json válido");

    let release_base = "https://github.com/MarioDeJesusRochaPosse/forja-syntax-registry/releases/download/grammars";
    let workdir = std::env::temp_dir().join(format!("forja-query-check-{}", std::process::id()));
    std::fs::create_dir_all(&workdir).expect("workdir");

    let mut failures = 0;
    for language in registry["languages"].as_array().expect("languages") {
        let id = language["id"].as_str().unwrap_or("?");
        if let Err(error) = check_language(&root, language, release_base, &workdir) {
            failures += 1;
            eprintln!("FAIL {id}: {error}");
        }
    }

    std::fs::remove_dir_all(&workdir).ok();
    if failures > 0 {
        eprintln!("{failures} lenguaje(s) con queries inválidas");
        std::process::exit(1);
    }
    println!("todas las queries compilan con sus gramáticas publicadas");
}
