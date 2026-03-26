; Taken from https://github.com/uben0/tree-sitter-typst/blob/f457c77edffd4b93190794355ff5acf7acfb99c6/editors/helix/queries/highlights.scm#L4
; Improved by @Gaspartcho
; CONTROL
(let
  "let" @keyword.storage.type)

(branch
  [
    "if"
    "else"
  ] @keyword.control.conditional)

(while
  "while" @keyword.control.repeat)

(for
  [
    "for"
    "in"
  ] @keyword.control.repeat)

(import
  "import" @keyword.control.import)

(as
  "as" @keyword.operator)

(include
  "include" @keyword.control.import)

(show
  "show" @keyword.control)

(set
  "set" @keyword.control)

(return
  "return" @keyword.control)

(flow
  [
    "break"
    "continue"
  ] @keyword.control)

; OPERATOR
(in
  [
    "in"
    "not"
  ] @keyword.operator)

(context
  "context" @keyword.control)

(and
  "and" @keyword.operator)

(or
  "or" @keyword.operator)

(not
  "not" @keyword.operator)

(sign
  [
    "+"
    "-"
  ] @operator)

(add
  "+" @operator)

(sub
  "-" @operator)

(mul
  "*" @operator)

(div
  "/" @operator)

(cmp
  [
    "=="
    "<="
    ">="
    "!="
    "<"
    ">"
  ] @operator)

(fraction
  "/" @operator)

(fac
  "!" @operator)

(attach
  [
    "^"
    "_"
  ] @operator)

(wildcard) @operator

; VALUE
(ident) @variable

(raw_blck
  "```" @punctuation.delimiter
  (blob) @text.literal)

(raw_blck
  lang: (ident) @tag)

(raw_span
  "`" @punctuation.delimiter
  (blob) @text.literal)

(label) @tag

(ref) @tag

(number) @number

(string) @string

(content
  [
    "["
    "]"
  ] @operator)

(bool) @boolean

(none) @constant.builtin

(auto) @constant.builtin

; Functions
(formula
  (ident) @function.method)

(attach
  (ident) @function.method)

(formula
  (field
    (ident) @function.method))

(tagged
  field: (ident) @tag)

(field
  field: (ident) @tag)

(call
  item: (ident) @function)

(call
  item: (field
    field: (ident) @function.method))

; MARKUP
(item
  "-" @punctuation.list_marker)

(term
  [
    "/"
    ":"
  ] @punctuation.list_marker)

(heading) @title

(url) @tag

(emph) @emphasis

(strong) @emphasis.strong

(symbol) @operator

(shorthand) @constant.builtin

(quote) @markup.quote

(align) @operator

(linebreak) @constant.builtin

(math
  "$" @operator)

"#" @operator

"end" @operator

(escape) @constant.character.escape

[
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  ";"
  ".."
  ":"
  "sep"
] @punctuation.delimiter

"assign" @punctuation

(field
  "." @punctuation)

(comment) @comment
