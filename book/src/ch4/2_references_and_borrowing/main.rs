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
    // - We change the variable to be mutable
    // - We create a mutable reference with &mut s
    // - We update the function signature to accept a mutable reference with some_string: &mut String
    let mut s = String::from("hello");
    change(&mut s);

    // Mutable references have a restriction
    // - There can only be one  mutable reference to piece of data at a time.
    let mut a = String::from("hello");
    // let r1 = &mut a;
    // compile time error: cannot borrow `a` as mutable more than once at a time
    // let r2 = &mut a; 
    // println!("{}, {}", r1, r2)

    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // A data race is similar to a reace condition and happens when:
    // - Two or more pointers access the same data at the same time
    // - At least one of the pointers is being used to write to the data
    // - There's no mechanism being used to synchronize access to the data
    //
    // We can use curly brackets to create a new scope, allowing for multiple mutable references,
    // just that not simultaneous ones

    {
        let _r1 = &mut a;
        // r1 goes out of scope here, so we can make a new reference with no problems
    }
    let _r2 = &mut a;


    // Combining Mutable and Immutable References
    //
    // let mut b = String::from("hello B");
    // let r3 = &b; // no problem
    // let r4 = &b; // no problem
    // let r5 = &mut b; // problem
                     // compile time error: cannot borrow `b` as mutable
                     // because it is also borrowed as immutable
    // println!("{}, {}, {}", r3, r4, r5);

    // Note that a reference's scope starts from where it is introduced and continues through the
    // last time that reference is used.
    // This code compiles because the last usage of the immutable references, the println!, occurs
    // before the mutable reference is introduced.

    let mut c = String::from("hello");

    let r1 = &c;
    let r2 = &c;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    
    let r3 = &mut c;    // This is valid.
    println!("{}", r3); // This is valid.

    // The ability of the compiler to tell that a reference is no longer being used at a point
    // before the end of the scope is called Non-Lexical Lifetimes (NLL).

    // Dangling References
    // - A pointer thata references a location in memory that may have been given to someone else,
    // by freeing some memory while preserving a pointer to that memory.
    // In Rust the compiler guarantees that references will never be dangling references.

    // let reference_to_nothing = dangle(); // compile time error
    let _reference_to_something = no_dangle(); // This is valid.

    // The Rules of References
    //
    // - At any given time, you can have either one mutable reference or any numbers of immutable
    // references
    // - References must always be valid.
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

fn change(some_string: &mut String) {
    some_string.push_str(", worldo");
}

// compile time error: this function's return type contains a borrowed value,
// but there is no value for it to be borrowed from
// fn dangle() -> &String {              // dangle returns a reference to a String
//     let s = String::from("hello");    // s is a new String
//     &s                                // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

// The solution is to return the string directly.
// Ownership is moved out, and nothing is deallocated.
fn no_dangle() -> String {
    String::from("hello")
}
