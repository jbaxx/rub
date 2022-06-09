// Methods are defined withn the contraxt of a struct, or an enum, or a trait object.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation Block
// - To define the function within the contract of Rectangle, we start an impl block for Rectangle.
// - Everything within this impl block will be associated with the Rectangle type.
// - In the signature of area, we use &self instead of rectangle: &Rectangle.
// - &self is short for self: &Self.
// - The type Self is an alias for the type that the impl block is for.
// - Methods must have a parameter named self of type Self for their first parameter.
// - We must use & in front of self to indicate this method borrows the Self instance.
// - Methods can:
//     - Take ownership of self: self (rare, usually used when the method transforms self into
//     something else and you want to prevent the caller from using the original instance after).
//     - Borrow self immutably: &self (like hear, we just want to read data, not write to it).
//     - Borrow self mutably: &mut self.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Implementation blocks are also used for organization.
    // We can find all the methods defined for a struct in the same place.
    // Also here, Rust knows when we mean the method widht() or the field width.
    // Rust doesn't implement getters by default.
    // Getters are useful because you can make the field private but the method public,
    // and thus enable read-only access to that field as part of the type's public API.
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    // All functions defiend within an impl block are called associated functions.
    // We can define associated functions that don't have self as their first parameter,
    // and thus are not methods because they don't need an instance of the type to work with.
    // For example: String::from.
    // These functions are often used for constructors.
    // square takes one dimension parameter and uses both as width and height,
    // to call this associated functio, we use the :: syntax.
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks
// Each struct is allowed to have multiple impl blocks.
// There's no reason to do this here, but it's valid syntax.
impl Rectangle {
    fn height(&self) -> bool {
        self.height > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels", rect1.area());
    // Also here, Rust knows when we mean the method widht() or the field width.
    println!("The rectangle has a nonzero width; it is {}", rect1.width());
    println!("The rectangle has a nonzero height; it is {}", rect1.height());

    // When you call a method with object.something(), Rust automatically adds in
    // &, &mut, or * so object matches the signature of the method.
    // In other words, the following are the same:
    // p1.distance(&p2);
    // (&p1).distance(&p2);
    // This is called automatic referencing and dereferencing.
    // The fact that Rust makes borrowing implicit for method receivers
    // is a big part of making ownership ergonomic in practice.

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("The square area is: {}", sq.area());
}
