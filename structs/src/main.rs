fn main() {
    println!("Hello, world!");

    let user1 = User {
	email: String::from("foo@bar.com"),
	username: String::from("foobar"),
	active: true,
	sign_in_count: 1,
    };

    println!("user1: {} {} {} {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let mut user2 = User {
	email: String::from("foo@bar.com"),
	username: String::from("foobar"),
	active: true,
	sign_in_count: 1,
    };

    user2.username = String::from("blip");
    user2.active = false;

    println!("user2: {} {} {} {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let user3 = build_user(String::from("a@b.com"), String::from("abc"));
    println!("user3: {} {} {} {}", user3.email, user3.username, user3.active, user3.sign_in_count);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
	email,
	username,
	active: true,
	sign_in_count: 1,
    }
}
