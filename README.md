# Adder Compiler

**Course:** CSCI 282L / Programming Languages

---

## Overview

Adder is a simple expression compiler written in Rust. It compiles a small S-expression-based language into x86-64 assembly. The generated assembly is then linked with a minimal Rust runtime that executes the code and prints the evaluated result.

## Supported Language Features

The compiler supports 32-bit signed integers and the following unary operations:

| Expression | Description | Example |
|---|---|---|
| `<number>` | Evaluates to the integer itself | `37`, `-42` |
| `(add1 <expr>)` | Adds 1 to the result of the expression | `(add1 5)` → `6` |
| `(sub1 <expr>)` | Subtracts 1 from the result of the expression | `(sub1 5)` → `4` |
| `(negate <expr>)` | Multiplies the result of the expression by -1 | `(negate 5)` → `-5` |

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
2. **Code Generator (`src/main.rs`)** — Recursively traverses the AST and emits corresponding x86-64 assembly instructions (`mov`, `add`, `sub`, `neg`), storing intermediate and final results in the `rax` register.
3. **Runtime (`runtime/start.rs`)** — A minimal Rust wrapper compiled as a C-callable executable. It calls the `our_code_starts_here` global label from the generated assembly and prints the returned 64-bit integer to standard output.

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

The `test/` directory contains a comprehensive suite of edge-case tests to verify the compiler's accuracy, including:

- **Standard evaluation** — `add`, `complex`, `nested`
- **Deeply nested scopes** — `deep_sub`, `cancel`
- **32-bit integer boundaries** — `max_int`, `min_int`
- **Whitespace parsing & negative literals** — `spaces`, `negative`

A complete execution transcript can be found in [`transcript.txt`](transcript.txt).
