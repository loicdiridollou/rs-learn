use core::fmt;

#[derive(Debug)]
struct User {
    user: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn main() {
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

    area_main()
}

fn build_user(user: String, email: String) -> User {
    User {
        user,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        return self.height * self.width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

impl Rectangle {
    fn square(size: i32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rectangle of dimensions (width {}, height {})",
            self.width, self.height
        )
    }
}

fn area_main() {
    let rectangle = Rectangle {
        width: 4,
        height: 6,
    };
    println!("The area of the rectangle is {}.", rectangle.area());
    println!("{}", rectangle);

    let rectangle2 = Rectangle {
        width: 8,
        height: 10,
    };

    if rectangle2.can_hold(&rectangle) {
        println!("Rectangle2 can be contained in rectangle")
    } else {
        println!("Rectangle2 cannot be contained in rectangle")
    };

    let square_rect = Rectangle::square(20);
    println!("{}", square_rect)
}
