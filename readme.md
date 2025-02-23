# Merc Interpreter

Merc is a simple interpreter for a dynamically-typed programming language. It supports basic programming constructs such as variables, functions, conditionals, loops, and arithmetic operations. The interpreter is written in Rust and provides a REPL (Read-Eval-Print Loop) for interactive programming.

## Features

- **Variables**: Define and manipulate variables using the `let` keyword.
- **Functions**: Define and call functions using the `func` keyword.
- **Conditionals**: Use `if` and `else` for conditional logic.
- **Loops**: Implement loops using the `while` keyword.
- **Arithmetic**: Perform basic arithmetic operations like addition, subtraction, multiplication, and division.
- **String Concatenation**: Concatenate strings using the `+` operator.
- **Boolean Operations**: Perform boolean operations like `&&` (AND) and `||` (OR).
- **REPL**: Interactive REPL for running code snippets.

### Example Script

Here is an example script that you can run with the interpreter:

```merc
let x = 10;
let y = 20;

func add(a, b) {
    return a + b;
}

let result = add(x, y);

if result > 30 {
    print("Result is greater than 30");
} else {
    print("Result is less than or equal to 30");
}

while x > 0 {
    print(x);
    x = x - 1;
}
```

### REPL Commands

- **help**: Show a list of available commands and language features.
- **clear**: Clear the screen and reset the REPL.
- **exit**: Exit the REPL.
- **env**: Show all defined variables in the current environment.

### Language Syntax

#### Variables

```merc
let x = 10;
let y = "Hello, world!";
```

#### Functions

```merc
func add(a, b) {
    return a + b;
}
```

#### Conditionals

```merc
if x > 10 {
    print("x is greater than 10");
} else {
    print("x is less than or equal to 10");
}
```

#### Loops

```merc
while x > 0 {
    print(x);
    x = x - 1;
}
```

#### Arithmetic

```merc
let sum = 1 + 2 * 3;
let difference = 10 - 5;
let product = 4 * 5;
let quotient = 20 / 4;
```

#### String Concatenation

```merc
let greeting = "Hello, " + "world!";
```

#### Boolean Operations

```merc
let is_true = true && false;
let is_false = true || false;
```