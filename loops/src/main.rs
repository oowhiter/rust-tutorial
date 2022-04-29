fn main() {
    // loop
    //
    loop {
        println!("again!");
        break;
    }

    // while
    //
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    // for
    //
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    for n in 0..20 {
        println!("fib {}: {}", n, fib(n));
    }
}

fn fib(n: i32) -> i32 {
    let (mut a, mut b) = (0, 1);
    if n == 0 {
        return a;
    } else if n == 1 {
        return b;
    }
    for _ in 1..n {
        (a, b) = (b, a + b);
    }
    b
}
