fn main() {
    // 4-1 ownership
    copy_variable();
    move_variable();
    clone_variable();
    call_function_with_copy();
    call_function_with_move();
    move_from_function_return();
    move_from_function_return_tuple();
    // 4-2 reference
    use_reference();
    reference_is_immutable();
    use_mutable_reference();
    references_in_scope();
    dangling_reference_is_not_compilable();
    // 4-3 slice
    bad_first_word();
    string_slice();
    good_first_word();
}

fn copy_variable() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn move_variable() {
    let s1 = String::from("hello");
    let s2 = s1;

    // // cannot use s1
    // println!("s1 = {}, s2 = {}", s1, s2);
    println!("s2 = {}", s2);
}

fn clone_variable() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn call_function_with_copy() {
    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }

    let x = 5;

    makes_copy(x);

    println!("x = {}", x);
}

fn call_function_with_move() {
    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    } // memory for some_string is freed

    let s = String::from("hello");

    // s is moved as with assignment
    takes_ownership(s);

    // // cannot use s
    // println!("s = {}", s);
}

fn move_from_function_return() {
    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // // cannot use s2
    // println!("s2 = {}", s2);

    println!("s3 = {}", s3);
}

fn move_from_function_return_tuple() {
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    // // cannot use s1
    // println!("s1 = {}", s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn use_reference() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn reference_is_immutable() {
    fn change(some_string: &String) {
        // // cannot mutate references
        // some_string.push_str(", world");
    }

    let s = String::from("hello");
    change(&s);
}

fn use_mutable_reference() {
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s = String::from("hello");
    change(&mut s);
}

fn references_in_scope() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);

    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    {
        // new scope
        let r1 = &mut s;
        println!("{}", r1);
    }

    println!("{}", &mut s);
    // println!("{}, {}", &mut s, &mut s);
    // println!("{}, {}", &mut s, &s);
    println!("{}", &s);
    println!("{}, {}", &s, &s);
    println!("{}, {}, {}", &s, &s, &s);
}

fn dangling_reference_is_not_compilable() {
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }
    //
    // let reference_to_nothing = dangle();

    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }

    let s = no_dangle();
}

fn bad_first_word() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", &s[..word]);

    s.clear();

    // // out of bounds
    // println!("{}", &s[..word]);
}

fn string_slice() {
    let s = String::from("hello");
    let len = s.len();

    let slice = &s[0..2];
    println!("string_slice: {}", slice);
    let slice = &s[..2];
    println!("string_slice: {}", slice);
    let slice = &s[3..len];
    println!("string_slice: {}", slice);
    let slice = &s[3..];
    println!("string_slice: {}", slice);
    let slice = &s[0..len];
    println!("string_slice: {}", slice);
    let slice = &s[..];
    println!("string_slice: {}", slice);
}

fn good_first_word() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let mut my_string = String::from("hello world");
    let word = first_word(&my_string[..]);

    // // cannot mutate because borrowed as immutable
    // my_string.clear();

    println!("{}", word);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    let word = first_word(my_string_literal);
    println!("{}", word);
}
