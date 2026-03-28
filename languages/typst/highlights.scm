; Taken from https://github.com/uben0/tree-sitter-typst/blob/f457c77edffd4b93190794355ff5acf7acfb99c6/editors/helix/queries/highlights.scm#L4
; Improved by @Gaspartcho
; CONTROL
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

(flow
  [
    "break"
    "continue"
  ] @keyword.control)

(return
  "return" @keyword.control)

; DIRECTIVES
(import
  "import" @keyword.control.import)

(wildcard) @operator

(as
  "as" @keyword.operator)

(include
  "include" @keyword.control.import)

(show
  "show" @keyword.control)

(set
  "set" @keyword.control)

(let
  "let" @keyword.storage.type)

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

(tagged
  field: (ident) @variable.parameter)

(field
  field: (_) @property)

; VALUE
(ident) @variable

(call
  item: (ident) @function)

(call
  item: (field
    field: (ident) @function.method))

; RAW
(raw_blck
  "```" @embedded
  (blob) @text.literal)

(raw_blck
  lang: (ident) @embedded)

(raw_span
  "`" @punctuation.delimiter
  (blob) @text.literal)

; MATH
[
  (label)
  (ref)
] @label

(number) @number

(string) @string

(bool) @boolean

(none) @constant.builtin

(auto) @constant.builtin

(formula
  (ident) @constant)

(formula
  (field
    (ident) @constant))

(attach
  (ident) @constant)

(attach
  (field
    (ident) @constant))

(attach
  [
    "^"
    "_"
  ] @operator)

(fraction
  "/" @operator)

(fac
  "!" @operator)

; MARKUP
(item
  "-" @punctuation.list_marker)

(term
  [
    "/"
    ":"
  ] @punctuation.list_marker)

(heading) @title

(url) @link_uri

(emph) @emphasis

(strong) @emphasis.strong

(symbol) @operator

(shorthand) @operator

(quote) @markup.quote

(code
  "#" @punctuation.special)

(math
  "$" @punctuation.special)

[
  (align)
  (linebreak)
] @punctuation.special

"end" @operator

(escape) @string.escape

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
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
