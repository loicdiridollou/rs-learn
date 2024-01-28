#[derive(Debug)]
struct User {
    user: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn structs_main() {
    let user1 = User {
        user: String::from("John"),
        email: String::from("j.d@gmail.com"),
        sign_in_count: 0,
        active: false,
    };

    let name = user1.user;
    println!("{}", name);
    let email = user1.email;
    println!("{}", email);
    let sign_in_count = user1.sign_in_count;
    println!("{}", sign_in_count);
    let active = user1.active;
    println!("{}", active);

    println!(
        "{:?}",
        build_user(String::from("John"), String::from("j.d@gmail.com"))
    );

    let user3 = User {
        user: String::from("new_name"),
        email: String::from("new_email"),
        ..user1
    };

    println!("{:?}", user3);
}

fn build_user(user: String, email: String) -> User {
    return User {
        user,
        email,
        sign_in_count: 1,
        active: true,
    };
}
