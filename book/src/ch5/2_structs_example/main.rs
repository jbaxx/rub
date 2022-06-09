fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area_1(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_2(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area_3(&rect2));

    // compile time error: `Rectangle` doesn't implement `std::fmt::Display`
    println!("The rect2 is {:?}", rect2);
    // The curly brackets tell println! to use formatting known as Display.
    // Rust doesn't know how we want to display the struct, printing the curly braces,
    // showing all fields?, printing commas?, etc.
    // Also, if we try to use the Debug print {:?} won't work due to the error:
    // `Rectangle` doesn't implement `Debug`
    // But we can opt-in for implicit Debug formatting by adding the outer attribute:
    // #[derive(Debug)]
    // just before the struct definition.
    // We can pretty print the debug using the formatting: {:#?}
    println!("The rect2 is:\n{:#?}", &rect2);

    // Another way to print out a value using the Debug format is to use the dbg! macro.
    // Which takes ownership of an expression, prints the file and line number
    // of where that dbg! macro call occurs in your code along with the resulting
    // value of that expression, and returns ownership of the value.
    // NOTE: calling dbg! prints to stderr.
    dbg!(&rect2);

    // We can put dbg! around the expression 30 * scale and,
    // because dbg! returns ownership of the expressions value,
    // the width field will get the same value as if we didn't have the dbg! call there.
    let scale = 2;
    let rect3 = Rectangle {
        // This prints: [src/ch5/2_structs_example/main.rs:40] 30 * scale = 60
        width: dbg!(30 * scale),
        height: 50,
    };
    // We don't want dbg! to take ownership of rect3 so we pass a reference.
    dbg!(&rect3);

}

// This function works but is not clear that the parameters are related,
// would be more manageable to group width and height together.
fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

// This function is better, tuples add a bit of structure.
// But this version is less clear, tuples don't name their elements.
// Mixing up the width and height wouldn't matter the area calculation,
// but if we want to draw the rectangle on the screen it would matter.
fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// With structs we add meaning to the data.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// The area function is now defined with one parameter, whose type is an immutable blorrow of a
// struct. We want to borrow rather than take ownership.
fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
