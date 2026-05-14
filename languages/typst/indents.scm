(_
  "["
  "]" @end) @indent

(_
  "{"
  "}" @end) @indent

(_
  "("
  ")" @end) @indent

(_
  "$"
  "$" @end) @indent

((comment) @indent
  (#match? @indent "^/\\*"))
