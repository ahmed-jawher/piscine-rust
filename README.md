# Piscine Rust

My solutions to the Reboot01 **Rust piscine**, an intensive bootcamp covering the Rust language from ownership basics to traits, generics, smart pointers and iterators. The repository contains **95 standalone Cargo crates**, one per exercise.

![Rust](https://img.shields.io/badge/Rust-2024%20edition-000000?logo=rust&logoColor=white)
![Cargo](https://img.shields.io/badge/Cargo-95%20crates-orange)

## Topics Covered

| Topic | Example exercises |
|---|---|
| **Ownership and borrowing** | `ownership`, `borrow`, `borrow_box`, `borrow_me_the_reference`, `copy`, `how_many_references`, `tuples_refs` |
| **Smart pointers and interior mutability** | `box_it`, `box_recursion`, `boxing_todo`, `ref_cell`, `drop_the_thread` |
| **Traits and generics** | `traits`, `easy_traits`, `generics`, `generics_list`, `lalgebra_scalar`, `lalgebra_vector` |
| **Closures and iterators** | `closures`, `iterators`, `collect`, `step_iterator`, `roman_numbers_iter` |
| **Lifetimes** | `lifetimes`, `delete_prefix` |
| **Error handling** | `error_types`, `question_mark`, `panic`, `unwrap_or_expect`, `doubtful`, `handling` |
| **Collections and hashing** | `hashing`, `simple_hash`, `slices_to_map`, `string_permutation`, `blood_types`, `commits_stats` |
| **Strings and text** | `strings`, `pangram`, `pig_latin`, `rot`, `cipher`, `scytale_cipher`, `edit_distance`, `profanity_filter`, `spelling` |
| **Math and matrices** | `matrix`, `matrix_mult`, `matrix_ops`, `matrix_transposition`, `fibonacci2`, `find_factorial`, `diamond_creation` |
| **Data modeling** | `card_deck`, `tic_tac_toe`, `shopping_mall`, `mobs`, `events`, `sales`, `project_motion`, `macro_calculator` |

## Running an Exercise

**Requirements:** Rust 1.85+ (the crates use the 2024 edition)

Each folder is an independent crate. Build or test one at a time:

```bash
git clone https://github.com/ahmed-jawher/piscine-rust.git
cd piscine-rust/roman_numbers
cargo build
cargo test
```

Most exercises are library crates (`src/lib.rs`); `looping` is an interactive binary:

```bash
cd looping && cargo run
```

## Notes

- A few crates use external dependencies: `chrono` (dates in `error_types` and `middle_day`) and `json` (`macro_calculator`, `boxing_todo`, `commits_stats`)
- `matrix`, `matrix_ops` and `lalgebra_scalar` depend on each other through local path dependencies
- There is no workspace manifest, so each crate builds on its own

## Author

**Ahmed Jawher** — [@ahmed-jawher](https://github.com/ahmed-jawher)
