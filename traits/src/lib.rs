#![allow(unused)]

mod overload {
    use std::ops::Add;
    // trait Add<RHS = Self> {
    //     type Output;
    //     fn add(self, rhs: RHS) -> Self::Output;
    // }

    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);

    struct Meters(u32);

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    #[test]
    fn overload() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
        assert_eq!(Millimeters(1000) + Meters(2), Millimeters(3000))
    }
}

mod method_conflict {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    #[test]
    fn method_conflict() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        // if Human::fly is not implemented, not compilable with error 'multiple applicable items in scope'
        person.fly();

        <Human as Pilot>::fly(&person)
    }

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    #[test]
    fn fullpath() {
        assert_eq!("Spot", Dog::baby_name());
        assert_eq!("puppy", <Dog as Animal>::baby_name());
    }
}

mod super_trait {
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    #[test]
    fn super_trait() {
        Point { x: 123, y: 4567 }.outline_print();
    }
}

mod new_pattern {
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }

    #[test]
    fn new_pattern() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
