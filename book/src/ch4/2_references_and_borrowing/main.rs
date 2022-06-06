// Passing a value and returning it inside a tuple is cumbersome.
// We can use a reference, is like a pointer that it's and addres  we can follow to access data
// stored at that address that is owned by som eother variable.
// It's guaranteed to point to a valid value of a particular type

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
