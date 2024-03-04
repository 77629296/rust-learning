#![allow(dead_code)]
#![allow(unused_variables)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_user() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

fn create_mut_user() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anothermail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn update_struct() {
    let user1 = build_user(String::from("build@mail.com"), String::from("name"));
    let user2 = User {
        email: String::from("another@email.com"),
        ..user1
    };
    println!("User email: {}", user2.email);
    // user1.username is moved to user2
    // println!("User email: {}", user1.username);
}

fn main() {
    // create_user();
    // create_mut_user();
    // build_user(String::from("build@mail.com"), String::from("name"));
    update_struct();
}
