## Package
A package is one or more crates that provide a set of functionality.
It contains a `Cargo.toml` file that descrives how to build those crates.

## Crate
A create can be a binary crate or a library crate.
The crate root is a source file that the Rust compiler starts from and makes up the root module of the crate.
NOTE: The content of either `src/main.rs` and `src/lib.rs` form a module named `crate` at the root of the crate's module structure.

## Modules Quick Reference
* Start from the create root: When compiling a create, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate).
* Declaring modules: In the crate root file, you can declare a new module named, say, "garden", with `mod garden;`. The compiler will look for the code inside the module in these places:
    * Inline, directly following `mod garden`, within curly brackets instead of the semicolon.
    * In the file `src/garden.rs`
    * In the file `src/garden/mod.rs`
* Declaring submodules: In any file other than the crate root that's being compiled as part of the crate (for example, `src/garden.rs`), you can declare submodules (for example, `mod vegetables;`). The compiler will look for the code inside submodules in these places within a directory named for the parent module:
    * Inline, directly following `mod vegetables`, within curly brackets instead of the semicolo.
    * In the file `src/garden/vegetables.rs`
    * In the file `src/garden/vegetables/mod.rs`
* Paths to code in modules: Once a module is being compiled as part of your crate, you can refer to code in that module (for example, an `Asparagus` type in the garden vegetables module) from anywhere else in this crate by using the path `crate::garden::vegetables::Asparagus` as long as the privacy rules allow.
* Private vs Public: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
* The `use` keyword: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and then only need to write `Asparagus` to make use of that type in the scope.

### Example of a binary crate named backyard
```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file: `src/main.rs`:
```
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

The `src/garden.rs` file:
```
pub mod vegetables;
```

The `src/garden/vegetables.rs` file:
```
#[derive(Debug)]
pub struct Asparagus {}
```

## Grouping related code in modules
We can create a new library crate with `cargo new --lib restaurant`
The `src/lib.rs` file:
```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

NOTE: The content of either `src/main.rs` and `src/lib.rs` form a module named `crate` at the root of the crate's module structure.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

```

## Paths
