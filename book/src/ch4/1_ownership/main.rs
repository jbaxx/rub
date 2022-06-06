// Ownership
// - Ownership is a set of rules tha governs how a Rust program manages memory
// - None of the features of ownership will slow down your program while it's running.

// The Stack and the Heap
// Stack
// - Stores values in the order it gets them and removes the values in the opposite order
// - last in, furst out
// - Adding data is called pushing onto the stack, removing data is popping off the stack
// - All data stored on the stack must have a known fixed size.
// - Data with unknown size at compile time or a size that might change must be stored on the heap.
// Heap
// - Less organized: when you put data on the heap, you request a certain amount of space
// - The memory allocator finds an empty spot in the heap that is big enough, marks it as in use,
// and returns a pointer to the location.
//
// - Pushing to the stack is faster than allocating on the heap, the allocator never has to search
// for a place to store new data, always at the top.
// - Allcoating on the heap requires more work, the allocator must find a big enough space to holde
// the data and then perform bookkeeping to prepera for the next allocation.
// - Accessing data in the heap is slower becasuse you have to follow a pointer.
// - Morden processors are faster if they jump around less in memory.
// - When your code calls a function, the values passed into the function (including, potentially,
// pointers to data on the heap) and the function's local variables get pushed onto the stack, when
// the function is over, those values get popped off the stack.
//
// Ownership addresses these problems:
// - Keeping track of what parts of code are using what data on the heap
// - Minimizing the amount of duplicate data on the heap
// - Cleaning up unused data on the heap so you don't run out of space

// Ownership Rules
// * Each value in Rust has a variable that's called its owner
// * There can only be one owner at a time
// * When the owner goes out of scope, the value will be dropped.

fn main() {

    // Variable Scope
    let _s = "hello";
    // The variable s refers to a string literal, the value is hardcoded into the text of our
    // program.
    
    // j is not valid here, it's not yet declared
    {
        let _j = "hello";  // j is valid from this point forward
                           //
                           // do stuff with j
                           //
    } // this scope is now over, and j is no longer valid
      // - When s comes into scope, it is valid
      // - It remains valid until it goes out of scope

    // The String Type
    // - Manages data allocated on the heap, thus can store an amount of text that is unknown to us
    // at compile time
    let _s = String::from("hello");
    // This kind of string can be mutated
    let mut s = String::from("hello");
    s.push_str(", worldo!"); // push_str() appends a literal to a String.
    println!("{}", s);

    // To support mutability and to grow, the String type needs to be allocated on the heap.
    // - The memory must be requested from the memory allocator at runtime
    // - We need a way of returning this memory to the allcoator when we're done with the String.
    // The first part is done by us (when we call String::from).
    // The second part is a difficult problem, if we forget, we'll waste memory, if we do it too
    // early well have an invalid variable, if we do it twice, it's a bug, we need to pair exactly
    // one allocate with exactly one free.
    // Rust takes a different path: memory is autamoatically returned once the variable that owns
    // it goes out of scope.
    //
    {
        let _s = String::from("hello"); // s is valid from the point forward
        // do stuff with s
        //
    }   // this scope is now over, and s is no longer valid
    //
    // When a variale goes out of scope, Rust calls a special function called drop.


    // Ways Variables and Data Interact: Move
    let x = 5;
    let _y = x;
    // As integers are simple values with known fixed size, both variables are allocated on the
    // stack and equal to 5
    let s1 = String::from("hello");
    let _s2 = s1; // Here s1 is no longer valid
    // println!("{}, world!", s1); // It will generate a compile time error: borrow of moved value: `s1`
    // s1 was moved into s2.
    // With only s2 valid, when it goes out of scope, it alone will free the memory, and we're
    // done.

    // Ways Variables and Dara Interact: Clone
    // If we want fo deep copy the heap data of the String, we can use the `clone` method.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // This works fine.

    // Stack-Only Data: Copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // This works fine.
    // This code doesn't call clone but works well.
    // Types such as integers, that have a known size at compile time are stored entirely in the
    // stack.
    // Then, copies of the actual value are quick to make, and is what happens.
    // These values implement the Copy trait, which allows  a variable to be still valid after
    // assignment to another variable.
    // Rust won't let us annotate a type with Copy if the type, or any of its parts has implemented
    // the Drop trait.
    //
    // Types that implement the Copy trait:
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Typles, if they only contain types that also implement Copy. For example: (i32, i32)

    // Ownership and Functions
    // Passing a variable to a function will move


}
