## How i made it -> book "Writing an interpreter in Go"

### Lexer

1. Define different types Tokens.
2. Write a character based lexer.
3. Write tests for it.
4. Initialize a REPL.

### Parsing

1. Think of every line as a Statement.
  - Eg: let <identifier> = <expression>;
2. Make Expression, Statement and Node(interface).
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
    NOTE: Max bugs are here cause I forget to assign precedence in token.rs file at end

  - for grouped expressions giving RBrace lowest precedence would give subtle bugs.
    (Main reason: as our parser is designed it treats it as Prefix function)

  - In If-else there were many off by one parsing errors beware.
    Eg:  if <condition> {consequence} else {alternative}

  - Functions:
    Eg: fn <parameters> <block statement>

    Fn calls:
    Eg: <expression>(<comma separated expressions>)

### Evalution 
  1. Create an Object enum to represent values.
  2. Evalute as per required logic.

    - make an TRUE,FALSE and NULL const obj values as these will not change so return 
      same each time reducing using extra memory.

    - Prefix Expression
      1. read prefix parser above.
      2. evaluate as per syntax and logic.
