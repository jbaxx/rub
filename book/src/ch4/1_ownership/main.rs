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

fn main() {}
