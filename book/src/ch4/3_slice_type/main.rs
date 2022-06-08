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
}
