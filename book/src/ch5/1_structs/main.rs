// Unlike tuples, structs name each piece of data so it's clear what the values mean.

struct User {
    active: bool,
    username: String,
    email: String,
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

    // To update a field
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
