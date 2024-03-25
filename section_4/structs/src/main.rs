struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

// Tuple based Struct
struct Coordinates(i32, i32, i32);

fn main() {
    let user1 = User {
        username: String::from("Joe"),
        active: true,
        sign_in_count: 0,
    };
    println!(
        "user1: {} {} {}",
        user1.username, user1.active, user1.sign_in_count
    );

    let user2 = build_user(String::from("Mary"));
    println!(
        "user2: {} {} {}",
        user2.username, user2.active, user2.sign_in_count
    );

    let cords = Coordinates(1, 2, 3);
    println!("cords: {} {} {}", cords.0, cords.1, cords.2);
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 0,
    }
}
