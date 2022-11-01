struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("roger@example.com"),
        username: String::from("roger"),
        active: true,
        sign_in_count: 69,
    };

    let user2 = User {
        email: String::from("roger2ya@example.com"),
        ..user1
    };

    user1.email = String::from("otherRoger@example.com");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the rectangle is {} square pixels.", rect1.area());
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
//
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1
//     }
