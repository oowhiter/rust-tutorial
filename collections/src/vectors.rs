pub fn make_vec1() -> Vec<i32> {
    let v: Vec<i32> = Vec::new();
    v
}

pub fn make_vec2() -> Vec<i32> {
    let v = vec![1, 2, 3, 4];
    v
}

pub fn update_vec1(v: &mut Vec<i32>) {
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

pub fn scope_drop() {
    {
        let v = vec![1, 2, 3, 4];
    }
    // // cannot use v and its elements?
    // println!("{}", v);
}

pub fn read_vec() {
    let v = vec![1, 2, 3, 4, 5];

    {
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
    }

    {
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // // mutable borrow cannot occur
    // v.push(6);
    println!("The first element is: {}", first);
}

pub fn iter_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

pub fn heterogeneous_vec() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(t) => println!("Text: {}", t),
        }
    }
}
