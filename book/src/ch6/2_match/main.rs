// match control flow construct
//
// Rust performs Pattern Matching.

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// When the match expression executes, it compares the resulting value against the Pattern
// of each arm.
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// To execute multiple lines of code in a match arm, use curly brackets.
fn value_in_cents_2(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Lucky Dime!");
            10
        },
        Coin::Quarter => 25,
    }
}

// Patterns that Bind to Values
// match arms can bind to the parts of the values that match the pattern.
// This is how we can extract values out of enum variants.
#[allow(dead_code)]
#[derive(Debug)]
enum Provincia {
    BocasDelToro,
    Cocle,
    // ...
}

#[allow(dead_code)]
enum Moneda {
    Penny,
    Nickel,
    Dime,
    Quarter(Provincia),
}

fn value_in_cents_3(moneda: &Moneda) -> u8 {
    match moneda {
        Moneda::Penny => 1,
        Moneda::Nickel => 5,
        Moneda::Dime => 10,
        Moneda::Quarter(prov) => {
            println!("Moneda de provincia {:?}!", prov);
            25
        }
    }
}

fn main() {

    let coin = Coin::Dime;

    println!("The value of {:#?} coin is: {}", coin, value_in_cents(&coin));
    println!("The value of {:#?} coin is: {}", coin, value_in_cents_2(&coin));
    println!("Moneda de: {}", value_in_cents_3(&Moneda::Quarter(Provincia::Cocle)));
}
