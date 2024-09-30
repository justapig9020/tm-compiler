# tm-compiler

## Introduction

`tm-compiler` compiles any Turing machine into a custom C-like intermediate representation (bf-c).

## Features

- Converts Turing machine definitions into a C-like IR.
- Provides a framework to represent Turing machines in a more accessible programming format.

## Installation and Usage

### Requirements

- `git`
- `cargo`

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/justapig9020/tm-compiler.git
   ```
2. Build the project:
   ```bash
   cargo build
   ```

## Examples

To compile a Turing machine into the C-like IR, use the following command:

```bash
cargo run -- ./sample/div.tm
```

This will compile a Turing machine that checks if 6 is divisible by 3 and outputs the C-like IR to the console.

To save the IR to a file, use redirection:

```bash
cargo run -- ./sample/div.tm > div.bfc
```

## Defining a Turing Machine

A Turing machine is defined in a text file, where each line represents a different definition. All symbols and states do not need additional declaration; `tm-compiler` will automatically interpret them.

### Definition Format

1. **First line**: The initial state of the tape, with different cells separated by spaces. Use `*` to mark the initial position of the read/write head. For example: `A *1 2 B` represents a tape with the initial state `[A, 1, 2, B]` and the read/write head pointing to the first cell (`1`).

2. **Second, third, and fourth lines**: Represent the initial state, accept state, and reject state, respectively.

3. **Subsequent lines**: Describe the transition functions in the format:
   ```
   <current state> <current symbol> <next symbol> <head movement (R / L)> <next state>
   ```
   For example, `q0 A B R q1` means that when in state `q0` and reading symbol `A`, the read/write head writes `B`, moves right (`R`), and the machine updates to state `q1`.

## Related Projects

This project is part of a series aimed at building a compiler to prove that Brainfuck is Turing complete. You can find the other related projects here:

- [tm-compiler](https://github.com/justapig9020/tm-compiler): Converts Turing machines into a custom C-like IR (bf-c).
- [bf-compiler](https://github.com/justapig9020/bf-compiler): Compiles bf-c programs into Brainfuck.
- [rubf](https://github.com/justapig9020/rubf): A Brainfuck virtual machine.

## License

tm-compiler is open-source and available under the MIT License. For more details, see the LICENSE file in the repository.
