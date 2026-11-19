# cli-calculator

A command-line calculator written in Rust. It evaluates arithmetic expressions with correct operator precedence, parentheses, and unary minus, and reports clear errors instead of crashing on bad input. It has no dependencies beyond the standard library.

## Features

- Operators: `+`, `-`, `*`, `/`
- Standard precedence (`2 + 3 * 4` is `14`) and left-to-right evaluation (`10 - 3 - 2` is `5`)
- Parentheses, nested to any depth: `(2 + 3) * 4`
- Unary minus: `-3 + 1.5`, `2 - -3`, `-(2 + 3)`
- Decimal numbers, with or without spaces: `12+3.5*(2-1)`
- Errors for invalid characters, malformed numbers, unexpected or missing tokens, division by zero, and overflow

## Requirements

Rust 1.85 or newer (the project uses the 2024 edition). Install it from [rustup.rs](https://rustup.rs).

## Usage

### Interactive mode

```
cargo run
```

Type an expression and press Enter. Type `q` (or press Ctrl-D / Ctrl-Z) to quit.

```
> 2 + 3 * 4
14
> (2 + 3) * 4
20
> 10 / 4
2.5
> 5 / 0
error: cannot divide by zero
> 2 +
error: unexpected end of input
> q
```

### One-shot mode

Pass the expression as arguments. The result is printed, or an error is written to stderr and the process exits with code 1.

```
cargo run -- 3 + 4
```

Your shell may treat `*`, `(`, and `)` specially, so quote the expression:

```
cargo run -- "2 * (3 + 4)"
```

## How it works

Evaluation is a three-stage pipeline, with each stage in its own module:

```
text  ->  tokenizer  ->  tokens  ->  parser  ->  number
```

| Module          | Responsibility                                                                         |
| --------------- | -------------------------------------------------------------------------------------- |
| `error.rs`      | `CalcError`, the single error type used by every stage                                 |
| `calculator.rs` | `Operator` and `calculate`, the arithmetic itself (divide-by-zero and overflow checks) |
| `tokenizer.rs`  | Turns text into `Token`s: numbers, operators, and parentheses                          |
| `parser.rs`     | Recursive-descent parser that evaluates the tokens and exposes `evaluate`              |
| `main.rs`       | Input and output only: the interactive loop and one-shot mode                          |

Dependencies point one way (`error` <- `calculator` <- `tokenizer` <- `parser` <- `main`), so there are no cycles.

### Grammar

The parser has one function per rule. Each rule is built from the one below it, which is what gives `*` and `/` higher precedence than `+` and `-`:

```
expression = term   (("+" | "-") term)*
term       = factor (("*" | "/") factor)*
factor     = "-" factor | NUMBER | "(" expression ")"
```

### Errors

Every failure is a `CalcError` value; the program does not panic on user input.

| Error              | Example input                |
| ------------------ | ---------------------------- |
| `InvalidCharacter` | `2 $ 3`                      |
| `InvalidNumber`    | `1.2.3`                      |
| `UnexpectedToken`  | `6 ** 2`, `(2 + 3 4`, `2 3`  |
| `UnexpectedEnd`    | `1 +`, `(2 + 3`, empty input |
| `DivideByZero`     | `5 / 0`                      |
| `Overflow`         | a result too large for `f64` |

## Development

```
cargo test       # run the unit and end-to-end tests
cargo clippy     # lints
cargo fmt        # formatting
```

Tests live next to the code they check: `calculator.rs` tests the arithmetic with plain numbers, `tokenizer.rs` tests text to tokens, and `parser.rs` tests precedence, parentheses, and full-pipeline behavior through `evaluate`.

## Limitations

- Numbers are `f64`, so results follow standard floating-point behavior (for example, `0.1 + 0.2` prints `0.30000000000000004`).
- Scientific notation (`1e5`) is not supported.
- Only the four operators above are supported.
- Errors report the first problem found, not every problem in the input.

## License

See [LICENSE](LICENSE).
