//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }
//

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
//
// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}


fn main() {
    // let m = Message::Write(String::from("hello"));
    // m.call();
    //
    // let some_number = Some(5);
    // let absent_number: Option<i32> = None;
    //

    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => move_player(other),
    // }

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
    // println!("five {:?}, six {:?}, none {:?}", five, six, none);

    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

}
