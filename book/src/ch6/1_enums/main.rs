#[allow(dead_code)]

// Example, any IP address can be either a version four or a version six, but not both at the same
// time.
// This concept can be expressed by defining an enumeration and listing the possible kinds an IP
// address can be.
enum IpAddrKind {
    V4,
    V6
}

// We can put data directly in enum variants.
// This avoids creating structs to hold this data, associated with the enums.
enum IpAddr {
    V4(String),
    // Each variant can have different types and amounts of associated data.
    V4Otherway(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
// Message is an enum that holds multiple variants.
enum Message {
    // Quit has no data associated with it at all.
    Quit,
    // Move has named fields like a struct does.
    Move { x: i32, y: i32 },
    // Write includes a single String.
    Write(String),
    // ChangeColor includes three i32 values.
    ChangeColor(i32, i32, i32),
}

// Equivalent to Message in structs.
#[allow(dead_code)]
struct QuitMessage; // Unit struct
#[allow(dead_code)]
struct MoveMessage {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
struct WriteMessage(String); // Tuple struct
#[allow(dead_code)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// If we used different structs, which each have their own type,
// we couldn't as easily define a funciton to take any of these kinds of messages,
// as we could with enums.

// We can also define methods on enums.
impl Message {
    fn call(&self) {
        // ...
    }
}


fn main() {

    // Variants of the enum are namespaced under its identifier.
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // The name of each enum variant also defines it's constructor.
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _otherhome = IpAddr::V4Otherway(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    

    // The Option Enum
    // Solves the case where a value could be something or nothing.
    // Avoids having null values.
    // Defined in the standard library as
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    #[allow(unused)]
    let some_number = Some(5);
    #[allow(unused)]
    let some_string = Some("a string");

    #[allow(unused)]
    // For none we need to anotate the type as can't be infered.
    let absent_number: Option<i32> = None;

    // unwrap extracts the value
    println!("some_number: {}", some_number.unwrap());
    // this will panic, better to use pattern matching,
    // or unwrap_or, unwrap_or_else, unwrap_or_default which will handle the None.
    // println!("absent_number: {}", absent_number.unwrap());
    println!("absent_number: {}", absent_number.unwrap_or(40));
}
