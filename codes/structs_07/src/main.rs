
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(active: bool, username: String, email: String, sign_in_count: u64) -> User{
    User {
        active,
        username,
        email,
        sign_in_count,
    }
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("username1234"),
        email: String::from("test@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("test1@gmail.com");


    println!("The struct types email is: {0}", user1.email);

    let user2 = build_user(false, String::from("user2"), String::from("user2@gmail.com"), 23);

    println!("User 2 mail information is: {0}", user2.email);

}
