fn main() {
    // 6-1
    use_enum();
    use_enum_in_struct();
    use_enum_with_data();
    use_enum_with_variant_data();
    use_enum_with_types();
    use_option();
    // 6-2
    use_match();
    // 6-3
    use_if_let();
}

fn use_enum() {
    enum IpAddrKind {
        V4,
        V6,
    }
    fn route(ip_type: IpAddrKind) {}

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(IpAddrKind::V6);
}

fn use_enum_in_struct() {
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn use_enum_with_data() {
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

fn use_enum_with_variant_data() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

fn use_enum_with_struct() {
    struct Ipv4Addr {}
    struct Ipv6Addr {}
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}

fn use_enum_with_types() {
    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            println!("Message called");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn use_option() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // // i8 and Option<i8> are different datatypes.
    // let sum = x + y;
    let sum = x + y.unwrap_or(0);
}

fn use_match() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: &Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let coin = Coin::Quarter(UsState::Alaska);
    println!("{:?}, {}", coin, value_in_cents(&coin));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // // match must be exhaustive.
    // fn _plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn use_if_let() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
    println!("{:?}", some_u8_value);

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("{}: State quarter from {:?}!", count, state),
        _ => count += 1,
    }
    println!("{:?}", coin);

    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    // Maybe we should use match expression for multiple arms ...
    } else if let Coin::Penny = coin {
        println!("{:?}", coin);
    } else {
        count += 1;
    }
    println!("{}: {:?}", count, coin);
}
