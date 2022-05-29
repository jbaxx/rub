fn main() {
    // Variables and Mutability
    //
    // This will generate a compilation error
    // Can't re-assign to immutable variable x
    //
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    //
    // This works:
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    // Constants
    // - Bound to a name and not allowed to change.
    // - Immutable by default.
    // - Must be type annotated.
    // - Naming convention: uppercase with underscores between words.
    //
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("How many seconds in an hour? {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    // - Declare a new variable with the same name as a previous variable.
    // - With every let we're actually creating a new variable
    // - It can even have different type
    let y = 6;
    println!("let y = 6; -> y is {}", y);
    let y = y * 2;
    println!("let y = y * 2; -> y is {}", y);
    {
        let y = 1423 * 2;
        println!("{{ let y = 1423 * 2; }} -> y in inner scope is {}", y);
    }
    println!("y in outer scope is still {}", y);
    let y = String::from("hello");
    // It can even be
    println!("let y = String::from(\"hello\"); y is {:?}", y);
}
