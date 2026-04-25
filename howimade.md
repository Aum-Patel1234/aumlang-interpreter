## How i made it -> book "Writing an interpreter in Go"

### Lexer

1. Define different types Tokens.
2. Write a character based lexer.
3. Write tests for it.
4. Initialize a REPL.

### Parsing

1. Think of every line as a Statement.
  Eg: let <identifier> = <expression>;
