## How i made it -> book "Writing an interpreter in Go"

### Lexer

1. Define different types Tokens.
2. Write a character based lexer.
3. Write tests for it.
4. Initialize a REPL.

### Parsing

1. Think of every line as a Statement.
  - Eg: let <identifier> = <expression>;
2. Make Expression, Statement and Node(interface)
  - Each program is a Vec<Statement>

  - Statement
    LetStatement -> `let <identifier> = <expression>;`
    ReturnStatement -> `return <expression>;`

  - Prefix statement-> <prefix operator><expression>;
    <prefix operator> -> only 2 `!, -`
  - Infix Statement -> <expression> <infix op> <expression>
  `FUN FACT: because of 2 operands left and right in infix statement they are 
            called binary expressions and prefix statement as unary expressions`

  - Main Parser Logic -> Pratt Parser

