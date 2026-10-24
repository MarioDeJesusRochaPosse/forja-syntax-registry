((line_comment) @comment)
((block_comment) @comment)
((string_literal) @string)
((raw_string_literal) @string)
((char_literal) @string)
((integer_literal) @number)
((float_literal) @number)
((boolean_literal) @constant.builtin)
((function_item name: (identifier) @function))
((call_expression function: (identifier) @function))
((type_identifier) @type)
((field_identifier) @property)
((identifier) @variable)
((macro_invocation macro: (identifier) @function.builtin))
[
  "as" "async" "await" "break" "const" "continue" "crate" "dyn"
  "else" "enum" "extern" "fn" "for" "if" "impl" "in" "let"
  "loop" "match" "mod" "move" "mut" "pub" "ref" "return" "self"
  "Self" "static" "struct" "super" "trait" "type" "unsafe" "use"
  "where" "while"
] @keyword
