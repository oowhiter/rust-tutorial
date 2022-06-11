#![allow(unused)]

mod if_let {
    /// `if let` syntax is not exhaustive.
    #[test]
    fn if_let() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
}

mod while_let {
    #[test]
    fn while_let() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
}

mod for_loop {
    #[test]
    fn for_loop() {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
}

mod let_statement {
    #[test]
    fn let_statement() {
        let x = 5;
        let (x, y, z) = (1, 2, 3);
        // let (x, y) = (1, 2, 3);
    }

    #[test]
    fn fn_args() {
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);
    }

    #[test]
    fn closure_args() {
        let f = |&(a, b)| {
            println!("{}, {}", a, b);
        };
        f(&(1, 2));
    }
}

mod refutability {
    #[test]
    fn refutability() {
        // // refutable pattern in local binding: `None` not covered
        // let Some(x) = Some(1);

        // // irrefutable `if let` pattern
        // if let x = 5 {
        //     //
        // }
    }
}

mod use_match {
    #[test]
    fn use_match() {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn match_scope() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    #[test]
    fn match_multi() {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn match_range() {
        let x = 5;
        match x {
            1..=5 => println!("one or two"),
            _ => println!("anything"),
        }

        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
}

mod decompose {
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn decompose_struct() {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    #[test]
    fn decompose_enum() {
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }
    }

    #[test]
    fn decompose_ref() {
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];

        let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
        let sum_of_squares: i32 = points.into_iter().map(|Point { x, y }| x * x + y * y).sum();
    }

    #[test]
    fn decompose_nested() {
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }
}

mod ignore {
    #[test]
    fn in_func_args() {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }
        foo(3, 4);
    }

    #[test]
    fn in_match() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                // 何らかの数値: {}, {}, {}
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }

        let s = Some(String::from("Hello!"));
        if let Some(_s) = s {
            println!("found a string");
        }
        // // already moved
        // println!("{:?}", s);

        let s = Some(String::from("Hello!"));
        if let Some(_) = s {
            println!("found a string");
        }
        // already moved
        println!("{:?}", s);
    }

    #[test]
    fn use_double_periods() {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
    }
}

mod use_ref {
    #[test]
    fn use_ref() {
        let robot_name = Some(String::from("Bors"));
        match robot_name {
            Some(name) => println!("Found a name: {}", name),
            None => (),
        }
        // // already moved
        // println!("robot_name is: {:?}", robot_name);

        let robot_name = Some(String::from("Bors"));
        match robot_name {
            Some(ref name) => println!("Found a name: {}", name),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);

        let robot_name = Some(String::from("Bors"));
        match &robot_name {
            Some(name) => println!("Found a name: {}", name),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);

        let mut robot_name = Some(String::from("Bors"));
        match robot_name {
            Some(ref mut name) => *name = String::from("Another name"),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);

        let mut robot_name = Some(String::from("Bors"));
        match &mut robot_name {
            Some(name) => *name = String::from("Another name"),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);
    }
}

mod match_guard {
    #[test]
    fn match_guard() {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {:?}", n),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);

        let x = 4;
        let y = false;
        assert_eq!(
            "no",
            match x {
                4 | 5 | 6 if y => "yes",
                _ => "no",
            }
        )
    }
}

mod at_operator {
    #[test]
    fn at_operator() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id @ 3..=7,
            } => {
                println!("Found an id in range: {}", id)
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            }
        }
    }
}
