pub fn make_string() {
    let s = String::new();
}

pub fn make_string_from_str() {
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);
}

pub fn make_string_encoded() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

pub fn push_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    assert_eq!(s, "foobar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    assert_eq!(s, "foobar");
    assert_eq!(s2, "bar");

    let mut s = String::from("lo");
    s.push('l');
    assert_eq!(s, "lol");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // // left operand must be owned
    // let s3 = &s1 + &s2;

    // signature:
    // fn add(self, s: &str) -> String {

    assert_eq!(s3, "Hello, world!");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    assert_eq!(s, "tic-tac-toe");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    assert_eq!(s, "tic-tac-toe");
}

pub fn get_part_of_string() {
    // // cannot access with index
    // let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "Здравствуйте";

    // // cannot compile
    // let answer = &hello[0];

    let s = &hello[0..4];

    // // panic
    // let s = &hello[0..3];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
