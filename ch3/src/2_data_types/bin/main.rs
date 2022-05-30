fn main() {
    // Scalar types
    //
    // Integer types (i: signed, u: unsigned)
    // 8-bit      i8      u8
    // 16-bit     i16     u16
    // 32-bit     i32     u32
    // 64-bit     i64     u64
    // 128-bit    i128    u128
    // arch       isize   usize     depends on the architecture the program is running on (32 or 64)
    //
    // Integer literals
    //
    // Decimal           98_222
    // Hex               0xff
    // Octal             0o77
    // Binary            0b1111_0000
    // Byte (u8 only     b'A)
    println!("This 8_123 is an integer literal using _ separators: {}", 8_123);
    //
    // Integer overflow
    //
    // This will throw an error if compiled on debug mode.
    //
    // let _x: u8 = 260;
    // error: literal out of range for `u8`
    //   --> src/2_data_types/bin/main.rs:20:18
    //    |
    // 20 |     let _x: u8 = 260;
    //    |                  ^^^
    //    |
    //    = note: `#[deny(overflowing_literals)]` on by default
    //    = note: the literal `260` does not fit into the type `u8` whose range is `0..=255`
    //
    // For release mode --release
    // Rust will perform two's complement wrapping, thus 256 becomes 0, 257 becomes 1, etc.
    //
    // Floating-Point types
    // - There are f32 and f64
    // Type inference defaults to f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // The below will automatically infer x type as f32, given its use.
    // let z = x + y;
    println!("x is {}, y is {}", x, y);

    // Numeric operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quiotient = 56.7 / 32.2;
    let _floored = 2 / 3; // results in 0

    // remainder
    let _remainder = 43 % 5;

    // Boolean type
    let _t  = true;
    let _f: bool = false;

    // Character type
    // - Specified with single quotes
    // - Four bytes
    // - Represents a unicode scalar
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

    // Compound types
    
    // Tuple type
    // - Groups together a number of values
    // - All can have differen types
    // - Have a fixed length when declared
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("A tuple printed with {{:?}}: {:?}", tup);
    println!("A tuple printed with {{:#?}}: {:#?}", tup);
    //
    // Tuple Pattern Matching (Destructuring)
    // To get the individual values,
    // we can use pattern matching to destructure
    let (x, y, z) = tup;
    println!("x is {}, y is {}, z is {}", x, y, z);
    //
    // Tuple dot notation element access
    println!("First element: {}, second element: {}, third element: {}", tup.0, tup.1, tup.2);

    // Unit type
    // - A tuple without elements: ()
    // - Expressions implicitly return the unit value if they don't return any other value.
    println!("Unit type: {:?}", ());
    
    // Array type
    // - All elements must have same type
    // - Type and length can be inferred
    let a = [1, 2, 3, 4, 5];
    //
    // Type declaration requires the type and length.
    let aa: [i32; 5] = [1, 2, 3, 4, 5];
    //
    // Array initialization if length is know for all values to a same value,
    // where the first element is the value and the second the length.
    let b = [3; 5];
    println!("Array a: {:?}", a);
    println!("Array aa: {:?}", aa);
    println!("Array b: {:?}", b);
    //
    // To access array elements:
    println!("Second element of the array a: {}", a[1]);
}
