// Passing a value and returning it inside a tuple is cumbersome.
// We can use a reference, is like a pointer that it's and addres  we can follow to access data
// stored at that address that is owned by som eother variable.
// It's guaranteed to point to a valid value of a particular type

fn main() {
    let s1 = String::from("hello");

    // The ampersands represent references,
    // they allow you to refer to some value without taking ownership
    // Note: the opposite of referencing
    // is dereferencing, using the operator *
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len); // This is valid.

    // Referencing
    // - We call the action of creating a reference borrowing
    // - References are immutable by default (as variables)
    // - We can't modify something we're borrowing
    // change(&s1); // This doesn't compile

    // Mutable References
    // This will work
    let mut s = String::from("hello");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what is refers to,
  // nothing happens.

// This doesn't compile with error:
// `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// fn change(some_string: &String) {
//     some_string.push_str(", world")
// }
