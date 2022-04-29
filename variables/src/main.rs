fn main() {
    //
    // Section 3.1
    //
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len();

    //
    // Section 3.2
    //

    let guess: u32 = "42".parse().expect("Not a number!");

    // int
    //
    let x: i8 = !(1 << 7);
    println!("i8: {}", x);
    let x: i16 = !(1 << 15);
    println!("i16: {}", x);
    let x: i32 = !(1 << 31);
    println!("i32: {}", x);
    let x: i64 = !(1 << 63);
    println!("i64: {}", x);

    let x: u8 = !0;
    println!("u8: {}", x);
    let x: u16 = !0;
    println!("u16: {}", x);
    let x: u32 = !0;
    println!("u32: {}", x);
    let x: u64 = !0;
    println!("u64: {}", x);
    let x: usize = !0;
    println!("usize: {}", x);

    let x = 98_222;
    println!("The value of x is: {}", x);
    let x = 0xff;
    println!("The value of x is: {}", x);
    let x = 0o77;
    println!("The value of x is: {}", x);
    let x = 0b1111_0000;
    println!("The value of x is: {}", x);
    let x = b'A';
    println!("The value of x is: {}", x);

    // float
    //
    let x = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // bool
    //
    let t = true;
    let f: bool = false;

    // char
    //
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    //
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // array
    //
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    let index = 10;
    // let element = a[index]; // compile error
    // println!("The value of element is: {}", element);   // 要素の値は{}です
}

const _MAX_POINTS: u32 = 100_000;
