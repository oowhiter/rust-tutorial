fn main() {
    // 10-1
    print_largest();
    extract_largest_func();
    largest_for_variant_types();
    largest_with_generics();
    struct_with_generic_type();
    struct_with_multi_generic_types();
    enum_with_generic_type();
    largest_with_generics_without_copy();
    // 10-3
    use_lifetime_annotation();
    use_lifetime_annotation_in_struct();
    use_static_lifetime();
}

fn print_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

fn extract_largest_func() {
    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn largest_for_variant_types() {
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

fn largest_with_generics() {
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn struct_with_generic_type() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let interger = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let interger_and_float = Point { x: 5, y: 4.0 };
}

fn struct_with_multi_generic_types() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let interger = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let interger_and_float = Point { x: 5, y: 4.0 };
    let mixup = Point { x: 5, y: 10.4 }.mixup(Point { x: "Hello", y: 'c' });
    println!("mixup.x = {}, mixup.y = {}", mixup.x, mixup.y);
}

fn enum_with_generic_type() {
    enum Case<T, U> {
        Case1(T),
        Case2(U),
    }
    let case: Case<u32, String> = Case::Case1(1);
}

fn largest_with_generics_without_copy() {
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn use_lifetime_annotation() {
    // The lifetime of the returned reference is the same as
    // the smaller of the lifetimes of the argument references.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    // // The lifetime of string2 is too short.
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

fn use_lifetime_annotation_in_struct() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // // `novel` does not live long enough.
    // let mut i = ImportantExcerpt{
    //     part: "",
    // };
    // {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //     i.part = first_sentence;
    // }
    // println!("{}", i.part);
}

// lifetime elision
// 1. lifetime parameter per each reference parameter
// 2. With a single input lifetime parameter, it will be assigned to all output lifetime parameters.
// 3. With lifetime of `self`, it will be assigned to all output lifetime parameters.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn use_lifetime_elision_for_method() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        // The lifetime of the returned reference is the same as that of `&self`.
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}

fn use_static_lifetime() {
    fn static_str() -> &'static str {
        let s = "I have a static lifetime.";
        return s;
    }
    let s = static_str();
    println!("{}", s);
}
