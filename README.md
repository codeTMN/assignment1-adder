# Boa Compiler

**Course:** CSCI 282L / Programming Languages

---

## Overview

Boa is an expression compiler written in Rust that builds on the foundations of Adder. It compiles a small S-expression-based language into x86-64 assembly, introducing **variables**, **let bindings**, **binary arithmetic**, and **stack-based memory management**. The generated assembly is then linked with a minimal Rust runtime that executes the code and prints the evaluated result.

## What Changed from Adder (Boa Changelog)

- **Stack-Based Memory** — Transitioned from purely register-based evaluation to using the x86-64 stack (`rsp` offsets) to store local variables and temporary values.
- **Environment Tracking** — Implemented a symbol table using `im::HashMap` to map variable names to stack offsets, enabling nested scopes and variable shadowing.
- **Binary Operations** — Added support for `+`, `-`, and `*`. Handled the complexity of saving the left operand to the stack to prevent register overwriting while evaluating the right operand, ensuring strict left-to-right evaluation.
- **Error Handling** — Added compile-time panics to catch invalid syntax, unbound identifiers, and duplicate variable bindings within the same scope.

## Supported Language Features

The compiler supports 32-bit signed integers, unary operations, variables, let bindings, and binary arithmetic:

| Expression | Description | Example |
|---|---|---|
| `<number>` | Evaluates to the integer itself | `37`, `-42` |
| `<identifier>` | Evaluates to the value bound to the variable | `x`, `y` |
| `(add1 <expr>)` | Adds 1 to the result of the expression | `(add1 5)` → `6` |
| `(sub1 <expr>)` | Subtracts 1 from the result of the expression | `(sub1 5)` → `4` |
| `(negate <expr>)` | Multiplies the result of the expression by -1 | `(negate 5)` → `-5` |
| `(+ <expr> <expr>)` | Adds two expressions | `(+ 3 4)` → `7` |
| `(- <expr> <expr>)` | Subtracts the second expression from the first | `(- 10 3)` → `7` |
| `(* <expr> <expr>)` | Multiplies two expressions | `(* 3 4)` → `12` |
| `(let ((<id> <expr>)+) <expr>)` | Binds one or more variables for use in the body expression | `(let ((x 5)) (+ x 1))` → `6` |

## Architecture Pipeline

```
┌─────────────┐     ┌──────────────────┐     ┌───────────┐
│  .snek file │ ──▶ │  Parser (AST)    │ ──▶ │  Code Gen │ ──▶  x86-64 .s
└─────────────┘     └──────────────────┘     └───────────┘
                                                                   │
                                                                   ▼
                                                          ┌────────────────┐
                                              Result ◀── │ Runtime + Link │
                                                          └────────────────┘
```

1. **Parser (`src/main.rs`)** — Reads the input `.snek` file and uses the `sexp` crate to parse the text into S-expressions. It then maps these into a custom Abstract Syntax Tree (AST).
2. **Code Generator (`src/main.rs`)** — Recursively traverses the AST and emits corresponding x86-64 assembly instructions (`mov`, `add`, `sub`, `neg`, `imul`), storing intermediate and final results in the `rax` register.
3. **Runtime (`runtime/start.rs`)** — A minimal Rust wrapper compiled as a C-callable executable. It calls the `our_code_starts_here` global label from the generated assembly and prints the returned 64-bit integer to standard output.
4. **Stack & Environment Management** — The compiler uses the `im::HashMap` crate to track variable environments, allowing for inner scope shadowing. Local variables are mapped to 8-byte stack offsets starting from `rsp - 16`. During binary operations, the left operand is temporarily saved to the stack to prevent register overwriting.

## System Requirements & Setup

To build and run this compiler, you will need:

- **Rust & Cargo** — for building the compiler and runtime
- **NASM** — for assembling the generated x86-64 code

### macOS (Apple Silicon) Note

Because the target output is x86-64 assembly, this project is configured to cross-compile the runtime on M-series Macs using Rosetta 
2. Ensure you have the proper Rust target installed:

```bash
rustup target add x86_64-apple-darwin
```

## Build and Execution

This project uses a **Makefile** to automate compiling the compiler, assembling the generated code, and linking the final executable.

To run a specific test file (e.g., `test/37.snek`):

```bash
make test/37.run
./test/37.run
```

To view the generated x86-64 assembly for a test:

```bash
cat test/37.s
```

## Testing

The `test/` directory contains a comprehensive suite of tests to verify the compiler's accuracy, including:

- **Variable shadowing & environment management** — `shadow`, `let_multi`
- **Left-to-right evaluation order & nested binary operations** — `sub_order`, `nested_arith`
- **Compiler panic error handling** — Duplicate bindings, Unbound variables, Invalid syntax
- **32-bit integer boundaries** — `max_int`, `min_int`
- **Whitespace parsing & negative literals** — `spaces`, `negative`

A complete execution transcript can be found in [`transcript.txt`](transcript.txt).
