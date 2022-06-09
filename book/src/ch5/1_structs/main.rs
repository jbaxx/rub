// Unlike tuples, structs name each piece of data so it's clear what the values mean.

struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}

fn main() {

    // Struct instantiation
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // To get the values we use dot notation
    println!("User 1 email: {}", &user1.email);

    // To mutate a field
    // - If the instance is mutable, we can change the value of a field
    // Note that the entire instance must be mutable.
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("second_someone@example.com");
    println!("User 2 email: {}", &user2.email);

    let usr = build_user(String::from("lol@lol.com"), String::from("loler"));
    println!("User 3 email: {}", &usr.email);

    // Struct Update Syntax
    // To create a new instance of a struct that includes mosst of the values from another.
    // NOTE: As email (which is String, heap type) is used to build the new struct,
    // the value is Moved here, thus the older instance (user1) is no longer valid.
    // If all the heap types were set and just the stack types (those that implement the Copy
    // trait), were used then it's a Copy instead of move and user1 could be valid after.
    let user3 = User {
        username: String::from("someone3@example.com"),
        // We use the .. syntax to signal from which instance to instantiate from.
        // It must come at the end.
        ..user1
    };
    println!("User 3 update email: {}", &user3.email);
    println!("User 3 update username: {}", &user3.username);
    // compile time error: due to user1 moved into user3
    // println!("User 1 email: {}, username: {}", &user1.email, &user1.username);

    // Tuple Structs without named fields
    // - They have the added meaning the struct name provides
    // - Tuple structs are useful when you want to give the whole tuple a name
    // and make the tuple a different type from other tuples.
    // These two tuples have the same types but different meaning.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-Like Structs without any fields
    // - They behave similar to ().
    // - Useful when you need to implement a trait on some type but don't have any data that you
    // want to store in the type itself.
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // Ownership of Struct Data
    //
    // We've used the owned String type rather than the &str string slice type.
    // This is deliberate choice because we want each instance of this struct
    // to own all of its data and for that data to be valid for as long as the entire struct
    // is valid.
    //
    // It's also possible for structs to store references to data owned by something else, but to
    // do so requires the use of lifetimes.
    // Lifetimes ensure that the data referenced by a struct is valid for as long as the struct it.
    // Let's say you tray to store a reference in a struct wihtout specifying lifetimes, like the
    // following won't work:
    //
    // compile time error: missing lifetime specifier
    // struct TheUser {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }
    //
    // let usern = TheUser {
    //     email: "someonen@example.com",
    //     username: "someunsername123",
    //     active: true,
    //     sign_in_count: 1,
    // };
}

// We can construct a new instance of the struct as the last expression in the function body
// to implicitly return that new instance.
fn build_user(email: String, usuario: String) -> User {
    User {
        email, // Shorthand for -> email: email,
        username: usuario,
        active: true,
        sign_in_count: 1,
    }
}
