# Diamondback Compiler

**Course:** CSCI 282L / Programming Languages

---

## Overview

Diamondback is an expression compiler written in Rust that builds on the foundations of Adder, Boa, and Cobra. It compiles a small S-expression-based language into x86-64 assembly, supporting **variables**, **let bindings**, **binary arithmetic**, **conditionals**, **tagged value representation**, **function definitions**, **function calls**, and **stack frame management**. The generated assembly is then linked with a minimal C runtime that executes the code and prints the evaluated result.

## What Changed from Adder (Boa Changelog)

- **Stack-Based Memory** — Transitioned from purely register-based evaluation to using the x86-64 stack (`rsp` offsets) to store local variables and temporary values.
- **Environment Tracking** — Implemented a symbol table using `im::HashMap` to map variable names to stack offsets, enabling nested scopes and variable shadowing.
- **Binary Operations** — Added support for `+`, `-`, and `*`. Handled the complexity of saving the left operand to the stack to prevent register overwriting while evaluating the right operand, ensuring strict left-to-right evaluation.
- **Error Handling** — Added compile-time panics to catch invalid syntax, unbound identifiers, and duplicate variable bindings within the same scope.

## Tagged Value Representation (Memory Scheme)

All values in this language are stored in **64-bit registers** but use the **Least Significant Bit (LSB)** as a type tag to distinguish between numbers and booleans at runtime.

| Type | Encoding Rule | LSB | Example |
|---|---|---|---|
| **Number** | Shifted left by 1 bit (`n << 1`) | `0` | `5` → `10` (`0b1010`) |
| **Boolean (false)** | Constant `1` | `1` | `false` → `1` (`0b01`) |
| **Boolean (true)** | Constant `3` | `1` | `true` → `3` (`0b11`) |

### Why Tag?

By reserving the LSB as a type tag, the compiler can pack type information directly into the value itself without needing a separate type field or wrapper. Numbers always have an LSB of `0` (because of the left shift), and booleans always have an LSB of `1`.

### Runtime Type Checking

Before executing any arithmetic operation (`+`, `-`, `*`, `add1`, `sub1`), the compiler emits a **bitwise AND 1** check on the operand:

```asm
test rax, 1       ; check LSB
jnz snek_error    ; if LSB != 0, value is not a number
```

If the LSB is **not `0`**, the value is a boolean (not a valid number), and the program safely jumps to a `snek_error` panic that reports an **"invalid argument"** to the user.

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

## Diamondback: Functions and Stack Frames

Diamondback introduces **user-defined functions** and **function calls**, bringing the language from a flat expression evaluator to something that actually feels like a real programming language. This required implementing a proper x86-64 calling convention with stack frame management — by far the most architecturally significant change so far.

### x86-64 Calling Convention

The compiler follows a structured caller/callee protocol to ensure that nested and recursive function calls don't corrupt each other's data.

#### Caller Responsibilities

- **Evaluates arguments right-to-left** and pushes each result onto the stack. This guarantees that the first argument ends up closest to the top of the callee's frame, matching the expected positive-offset access pattern.
- **Aligns the stack to 16 bytes** before issuing the `call` instruction. x86-64 requires 16-byte stack alignment at the point of a `call`, so the compiler inserts a padding `push` when the argument count is even (since `call` itself pushes an 8-byte return address).
- **Cleans up the stack** after the function returns by adding back to `rsp` to pop off all the arguments (and any alignment padding).

```asm
;; Example: calling f(1, 2, 3)
push 6          ; arg 3 (tagged: 3 << 1)
push 4          ; arg 2 (tagged: 2 << 1)
push 2          ; arg 1 (tagged: 1 << 1)
call fun_f
add rsp, 24    ; clean up 3 args × 8 bytes
```

#### Callee Responsibilities

- **Prologue** — Saves the caller's base pointer and establishes its own frame:
  ```asm
  push rbp
  mov rbp, rsp
  sub rsp, <locals * 8>   ; reserve space for local variables
  ```
- **Epilogue** — Tears down the frame and returns control to the caller:
  ```asm
  mov rsp, rbp
  pop rbp
  ret
  ```

This `push rbp` / `pop rbp` dance is what allows arbitrarily deep call chains (including recursion) to work without stepping on each other's memory.

### Memory Layout

The stack frame is split into two regions relative to `rbp`:

| Region | Offset from `rbp` | Description |
|---|---|---|
| Return address | `[rbp + 8]` | Pushed automatically by `call` |
| Parameter 1 | `[rbp + 16]` | First argument passed by the caller |
| Parameter 2 | `[rbp + 24]` | Second argument |
| Parameter *n* | `[rbp + 8*(n+1)]` | *n*-th argument |
| Local variable 1 | `[rsp + 8]` | First let-bound variable |
| Local variable 2 | `[rsp + 16]` | Second let-bound variable |

- **Parameters** are accessed via **positive offsets from `rbp`**, since the caller pushed them *before* the `call` instruction pushed the return address and *before* the callee pushed `rbp`.
- **Local variables** are mapped to **positive offsets from `rsp`** (which sits below `rbp` after the `sub rsp, ...` in the prologue). This prevents nested function calls from overwriting locals — each callee gets its own frame below the current `rsp`.

### Built-in `print` Function

Diamondback adds a built-in `print` function implemented as a C-interop routine linked from the runtime. It dynamically unwraps the tagged representation at runtime to produce human-readable output:

- If the LSB is `0`, the value is a **number** — it shifts right by 1 and prints the integer.
- If the value is `3`, it prints `true`.
- If the value is `1`, it prints `false`.

```c
// Simplified logic from the runtime
if (val == 3) {
    printf("true\n");
} else if (val == 1) {
    printf("false\n");
} else if ((val & 1) == 0) {
    printf("%lld\n", val >> 1);
}
```

Unlike `snek_error`, `print` does **not** halt execution — it outputs the value and returns it, making it usable inline within expressions (e.g., `(print (+ 1 2))` prints `3` and evaluates to `3`).

---

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
