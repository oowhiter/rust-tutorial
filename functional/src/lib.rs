#![allow(unused)]

mod fn_pointer {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    #[test]
    fn fn_pointer() {
        let answer = do_twice(add_one, 5);
        assert_eq!(12, answer);

        let answer = do_twice(|x| x + 1, 5);
        assert_eq!(12, answer);

        // let y = 1;
        // let answer = do_twice(move |x| x + y, 5);
        // assert_eq!(12, answer);
    }

    #[test]
    fn mapper() {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
    }

    // // not compilable
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    #[test]
    fn use_closure() {
        let f = returns_closure();
        assert_eq!(2, f(1));
    }
}
