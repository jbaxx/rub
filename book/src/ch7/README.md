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
????????? Cargo.lock
????????? Cargo.toml
????????? src
    ????????? garden
    ???   ????????? vegetables.rs
    ????????? garden.rs
    ????????? main.rs
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
 ????????? front_of_house
     ????????? hosting
     ???   ????????? add_to_waitlist
     ???   ????????? seat_at_table
     ????????? serving
         ????????? take_order
         ????????? serve_order
         ????????? take_payment

```

## Paths for Referring to an Item in the Module Tree
A path can take two forms:
* An *absolute path* starts from a crate root by using a crate name (for code from an external crate) or a literal `crate` (for code from the current crate).
* A *relative path* from the current module and uses `self`, `super`, or an identifier in the current module.

```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

NOTE: Items in a parent module can't use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
Child modules wrap and hide their implementation details, but the child modules can see the context in which they're defined.

## Making a module public doesn't make its contents public.
For the `eat_at_restaurant` function to compile, we need to provide public visiblity to all the members in the module tree down to the `add_to_waitlist` function.
```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

See more at [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Recommendation
For packages with both binary and library crates root, the module tree should be defined in `src/lib.rs`, the any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate.

## Starting Relative Paths with `super`
Using `super` at the start of the path is like starting a filesystem path with the `..` syntax.
```
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

## Making Structs and Enums Public
If we use pub before a struct definition, we make the struct public, but the struct's fields will still be private.
We can make each field public or not on a case-by-case basis.
In the example the `toast` field is public and can be accessed and modified in the outter function, but the field `seasonal_fruit` isn't.
NOTE: as `back_of_house::Breakfast` has a private field, we need a public function (constructor) to construct it, as we won't be able to create an instance of `Breakfast` otherwise.
```
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

In contrast, if we make an enum public, all of its variants are then public.
```
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## Bringing Paths into Scope with the `use` keyword
