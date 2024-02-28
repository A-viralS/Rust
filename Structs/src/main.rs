struct User {
    email: String,
    user_name: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("u1@gmail.com"),
        user_name: String::from("u1"),
        sign_in_count: 1,
        active: true,
    };
    println!("name of the user is {}", user1.user_name);

    user1.user_name = String::from("u1Changed");
    println!(" changed name of the user is {}", user1.user_name);

    let user2 = create_user(String::from("u2"), String::from("u1@gmail.com"));
    println!("name of the seconc user is {}", user2.user_name);
}

fn create_user(user_name: String, email: String) -> User {
    User {
        user_name,
        email,
        active: true,
        sign_in_count: 1,
    }
}
