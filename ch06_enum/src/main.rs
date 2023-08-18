fn main() {
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        route(four);
        route(six);
    }

    {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    {
        let home = IpAddr_2::V4(String::from("127.0.0.1"));
        let loopback = IpAddr_2::V6(String::from("::1"));
    }

    {
        let home = IpAddr_3::V4(127, 0, 0, 1);
        let loopback = IpAddr_3::V6(String::from("::1"));
    }

    {
        let m = Message::Write(String::from("hello"));
        m.Call();
    }

    {
        /* 标准库定义的枚举,代表null与非null
        enum Option<T> {
            None,
            Some(T),
        }
        */
        let some_number = Some(5);
        let some_char = Some('e');
        let absent_number: Option<i32> = None;
    }

    {
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    //match
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}
    }

    //match
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
    }

    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => (),
        }
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        } else {
        }
    }
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr_2 {
    V4(String),
    V6(String),
}

enum IpAddr_3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    /*
    struct Quit;
    struct Move {
        x: i32,
        y: i32,
    }
    struct Write(String);
    struct ChangeColor(i32, i32, i32);
     */
}

impl Message {
    fn Call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
