fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("lawldewm"),
        email: String::from("emaile@me.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("email@you.com");

    let user2 = User {
        email: String::from("user@email.com"),
        ..user1 //moves all ptrs to user2, user1.email still valid
    };
    let user3 = User {
        email: String::from("three@email.com"),
        username: String::from("three"),
        ..user2 //copies sign in and active, user2 still valid
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn user_constructor(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;
