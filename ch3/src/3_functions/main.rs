
fn main() {
    println!("Hellow, world");

    // Functions
    //
    // - Naming convention: snake_case.
    // - Defined by entering fn, followed by a name, and a set of parentheses.
    another_function();

    // Parameters
    // - Parameters is the definition of what the function expects
    // - Arguments are the concrete values passed to the function
    // - People tend to use parameters and arguments interchangeably
    another_function_param(54);
    print_labeled_measurement(48, 'h');

    // Statements and expressions
    // - Statements are instructions that perform some action and DO NOT return a value
    // - Expressions evaluate to a resulting value.
    // - Function bodies are made up of a series of statements optionally ending in an expression.
    //
    // Statements:
    // let y = 6;
    // Not Statement (compile error)
    // let x = (let y = 6); // note: variable declaration using `let` is a statement
    //
    // Expressions:
    // println!("hello");
    // { let x = 3; x + 1 } // evaluates to 4, thus:
    let _y = { let x = 3; x + 1 }; // is valid
    println!("Result from expression from scope block: {}", _y);
    // - expressions like these doesn't end in semicolon, otherwise we turn it into statements.

    // Functions with Return Values
    // - Return values aren't named
    // - Type is declared with ->
    // - The return value is synonymous with the final expression in the block of the function body
    println!("{}", five());
    let x = five();
    println!("The value of x is {}", x);
    let y = plus_one(5);
    println!("The value of y is {}", y);
    let z = plus_one_unit(5);
    println!("The value of z is {:?}", z);
}

fn five() -> i64 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_one_unit(x: i32) {
    let _ = x + 1;
}

fn another_function() {
    println!("Another funciton.");
}

fn another_function_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
