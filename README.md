# piscine-rust

A collection of small, self-contained Rust exercises written as part of a 42-style "piscine" (bootcamp), each living in its own Cargo crate.

## Overview

This repository is not a single application but a workspace of **79 independent Rust crates**, one per directory at the repository root. Each crate is a focused exercise on a specific Rust language feature or a small programming/algorithmic problem (e.g. `ownership`, `closures`, `lifetimes`, `error_types`, `matrix_ops`, `roman_numbers`). Almost every exercise is a **library crate** (`src/lib.rs`) exposing functions, structs, and traits; `looping` is the one exception, built as a binary (`src/main.rs`) that reads from stdin in a loop.

There is no root `Cargo.toml`/workspace manifest, no CI configuration, and no per-exercise `README`s — each crate stands alone with its own `Cargo.toml`.

## Features

Confirmed exercise topics present in the source, grouped by theme:

- **Ownership & borrowing**: `ownership`, `borrow`, `borrow_box`, `borrow_me_the_reference`, `copy`, `how_many_references`, `tuples_refs`
- **Smart pointers & interior mutability**: `box_it`, `box_recursion` (a linked list via `Box`), `ref_cell` (`RefCell`/`Rc`), `drop_the_thread` (`Cell`/`RefCell` state tracking)
- **Generics & traits**: `generics`, `generics_list`, `traits`, `easy_traits`
- **Closures & iterators**: `closures`, `iterators` (custom `Iterator` impl for a Collatz sequence), `collect`
- **Lifetimes**: `lifetimes`
- **Error handling**: `error_types` (custom error struct with `chrono` timestamps), `question_mark` (`?` operator chaining through nested `Option`s), `panic`, `unwrap_or_expect`, `doubtful`
- **Collections & hashing**: `hashing`, `simple_hash`, `string_permutation`, `blood_types`
- **Strings & text processing**: `strings`, `string_literals`, `reverse_string`, `capitalizing`, `pangram`, `rot` (ROT cipher), `cipher`, `profanity_filter`, `name_initials`, `edit_distance` (Levenshtein distance), `to_url`
- **Numbers & math**: `adding`, `adding_twice`, `bigger`, `division_and_remainder`, `find_factorial`, `fibonacci2`, `scalar`, `temperature_conv`, `circle`, `logic_number`, `middle_day`
- **Matrices & linear algebra**: `matrix`, `matrix_mult`, `matrix_ops`, `matrix_transposition`, `lalgebra_scalar` (with `matrix_ops` and `matrix` depending on sibling crates via path dependencies)
- **Data modeling / small programs**: `card_deck` (playing-card suits/ranks with tests), `tic_tac_toe`, `shopping_mall`, `groceries`, `sales`, `scores`, `get_products`, `arrange_it`, `arrays`, `banner`, `stars`, `talking`, `handling`, `changes`, `speed_transformation`, `searching`, `highest`, `ordinal`, `does_it_fit` (geometry: areas/volumes), `vector_operations`, `macro_calculator` (nutrition macro totals, serialized to JSON), `looping` (interactive stdin loop)
- **Roman numerals**: `roman_numbers`, `roman_numbers_iter`

Note: despite its name, `macro_calculator` computes nutritional "macros" and does not use Rust's `macro_rules!` — no exercise in the repository defines declarative or procedural macros, and none use `unsafe` or OS threads.

## Technologies

- **Rust** — 78 of the 79 crates target the **2024 edition**; `sales` targets the **2021 edition**. No `rust-toolchain` file is present.
- **Cargo** for building and dependency management (no workspace manifest; each crate is built independently)
- External crates used by specific exercises (declared in the relevant `Cargo.toml`):
  - [`chrono`](https://crates.io/crates/chrono) — used by `error_types` and `middle_day` for date/time handling
  - [`json`](https://crates.io/crates/json) — used by `macro_calculator` to build a `JsonValue`
  - Local path dependencies: `matrix_ops` depends on `matrix`, and `matrix` depends on `lalgebra_scalar`

All other crates declare an empty `[dependencies]` section and rely only on the Rust standard library.

## Project Structure

```
piscine-rust/
├── adding/                  # each exercise is its own crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── adding_twice/
├── arrange_it/
├── ...
├── matrix/                  # example of crates linked via path dependencies
├── matrix_ops/              #   -> depends on ../matrix
├── lalgebra_scalar/         #   -> depended on by ../matrix
├── looping/                 # the one binary crate (src/main.rs)
├── LICENSE
└── COPYRIGHT.md
```

Each exercise directory contains its own `Cargo.toml` and a `src/` folder. A few exercises split logic across more than one file (e.g. `does_it_fit/src/areas_volumes.rs`, `matrix/src/{mult,ops}.rs`, `ref_cell/src/messenger.rs`, `shopping_mall/src/mall.rs`), but the public API of each crate is re-exported from its `lib.rs`.

## Requirements

- Rust toolchain capable of building the **2024 edition** (Rust 1.85 or newer) to build most crates; `sales` only requires 2021-edition support.
- Cargo (bundled with the Rust toolchain).
- Internet access on first build for crates with external dependencies (`error_types`, `middle_day`, `macro_calculator`) so Cargo can fetch `chrono` and `json`.

## Installation

```bash
git clone https://github.com/3xoob/piscine-rust.git
cd piscine-rust
```

There is no top-level build step — build/run/test each exercise from within its own directory.

## Usage

Each exercise is built and run independently, from inside its directory. For example:

```bash
cd ownership
cargo build

cd ../looping
cargo run
```

Since `looping` is the only binary crate, `cargo run` only produces runnable output there; every other crate is a library and is exercised via `cargo test`, `cargo build`, or by depending on it from another crate (as `matrix_ops` does with `matrix`).

## Testing

Only one exercise ships automated tests: `card_deck`, which has a `#[cfg(test)]` module covering suit/rank translation, invalid-value panics, and winning-card detection. Run it with:

```bash
cd card_deck
cargo test
```

The remaining 78 crates have no `#[test]` functions.

## Example

`card_deck/src/lib.rs` models a playing card and checks for a winning hand:

```rust
pub fn winner_card(card: Card) -> bool {
    matches!((card.suit, card.rank), (Suit::Spade, Rank::Ace))
}
```

`question_mark/src/lib.rs` chains the `?` operator through four levels of nested `Option`s:

```rust
impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        self.first_layer?.second_layer?.third_layer?.fourth_layer
    }
}
```

## Learning Objectives

Based on the exercises actually present, this piscine covers:

- Core ownership, borrowing, and copy semantics
- Smart pointers (`Box`), interior mutability (`Cell`, `RefCell`), and reference counting (`Rc`)
- Generics, trait definitions, and trait bounds
- Lifetime annotations
- Closures and implementing the `Iterator` trait
- Idiomatic error handling with custom error types and the `?` operator
- Working with `String`/`&str`, collections (`Vec`, `HashMap`), and hashing
- Small numerical and algorithmic problems (factorial, Fibonacci, edit distance, Roman numerals, matrix/linear-algebra operations)
- Structuring a multi-crate project where some crates depend on sibling crates via path dependencies
- Integrating a small external crate (`chrono` for dates, `json` for serialization)

## Limitations

- No workspace `Cargo.toml` ties the crates together; each must be built/tested individually.
- Automated test coverage is minimal — only `card_deck` has tests.
- There is no CI configuration in the repository.
- No per-exercise documentation/subject files are included; the intent of each exercise must be inferred from its source.

## License

This repository includes a `LICENSE` file and a `COPYRIGHT.md`. The code is copyrighted (all rights reserved) and made available publicly for portfolio/viewing purposes only — no permission is granted to copy, modify, distribute, or reuse it without prior written permission from the copyright holder. See `LICENSE` for the full text.
