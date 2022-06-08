// Example program
// Write a function that takes a string and returns the first word it finds in that string.
// If the function doesn't find a space in the string, the whole string must be one word.

// We could return the index of the end of the word indicated by a space.
fn first_word(s: &String) -> usize {
    // As we need to go through the String element by element and check whether a value is a space,
    // we'll convert our String to an array of bytes using the as_bytes method.
    let bytes = s.as_bytes(); 

    // We create an iterator over the array of bytes using the iter method.
    // The first element of the tuple returned by enumerate is the index,
    // and the second is a reference to the element.
    // As we get a reference, we use & in the pattern we use to descontruct the tuple.
    for (i, &item) in bytes.iter().enumerate() {
        // To look for the bye we use the byte literal syntax.
        // If we find a space, we return the position, otherwise we return the length.
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
// We're returninga usize, but it's only a meaningful number in the context of the &String.
// There's no guarantee it will be valid in the future as well.

fn main() {
    let mut s = String::from("hello worldo"); // The mutable is needed for the clear method.

    let word = first_word(&s); // word will get the value 5

    println!("First word at index: {}", word);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that we could
    // meaningfully use the value 5 with. word is now totally invalid!
    println!("First word at index: {}", word);

    // The program compiles without any errors.
    //
    // We're calculating values from data in a particular state that aren't tied to that state at
    // all.

    // String Slices
    // - A string slice is a reference to a part of a String

    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // Rather than a reference to the entire String, hello is a reference to a portion.
    // We create slices using a range within brackets: [starting_index..ending_index]
    // Where ending_index is one more than the last position in the slice

    // Range syntax
    // To start at index zero you can drop the value before the two periods.
    let _slice = &s[0..2];
    let _slice = &s[..2];
    // As well you can drop the trailing number
    let len = s.len();
    let _slice = &s[3..len];
    let _slice = &s[3..];
    // You can drop both values to take a slice of the entire String.
    // These are equal:
    let _slice = &s[0..len];
    let _slice = &s[..];
    // Note: String slice range indices must occure at valid UTF-8 character boundaries.
    // Otherwise the program may exit with an error.

    let mut c = String::from("hello world");
    let _word = first_word_word(&c);
    // c.clear(); // compile time error: as its a mutable borrow when an immutable reference exist.
                  // and the println! uses the immutable reference later.
    // println!("the first word is: {}", word);

    // String Literals Are Slices
    let s = "Hellow, rodl!";
    // The type of s here is &str, it's a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable, &str is an immutable reference.

    // String Slices as Parameters
    // Knowing that you can take slices of literals and String values.
    // The signature:
    //     fn first_word(s: &String) -> &str
    // Can be written as:
    //     fn first_word(s: &str) -> &str
    // As it allows to use the same function on both &String and &str.
    // - If we have a string slice, we can pass it directly
    // - If we have a String, we can pass a slice of the String or a reference tot he String.
    // This flexibility takes advantage of deref coercions.

    let my_string = String::from("hello world");

    // function works on slices of String, whether partial or whole
    let _word = first_word_word(&my_string[0..6]);
    let _word = first_word_word(&my_string[..]);
    // works on references so String, which are equivalent to whole slices of String
    let _word = first_word_word(&my_string);

    let my_string_literal = "hello world";
    // function works on slices of string literal, wheter partial or whole
    let _word = first_word_word(&my_string_literal[0..6]);
    let _word = first_word_word(&my_string_literal[..]);
    // because string literals *are* string slices already,
    // this works too, wihtout the slice syntax
    let _word = first_word_word(&my_string_literal);

    // Other Slices
    // String slices are specific to strings, but there's a more general slice type.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    // This slice has the type &[i32].
}

// Rewriting the function
fn first_word_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // &s[..]
    s
}
