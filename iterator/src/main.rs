fn main() {
    println!("Hello, world!");
}

#[test]
fn iterate() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    println!("{:?}", v1);
    // // borrow of moved value
    // println!("{:?}", v1_iter);
}

#[test]
fn demonstrate_iter() {
    let v1 = vec![1, 2, 3];

    // pub fn iter(&self) -> Iter<'_, T>
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    println!("{:?}", v1);
    println!("{:?}", v1_iter);
}

#[test]
fn demonstrate_into_iter() {
    let v1 = vec![1, 2, 3];

    // fn into_iter(self) -> Self::IntoIter
    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);

    // // borrow of moved value
    // println!("{:?}", v1);
    println!("{:?}", v1_iter);
}
#[test]
fn demonstrate_iter_mut() {
    let mut v1 = vec![1, 2, 3];

    // pub fn iter_mut(&mut self) -> IterMut<'_, T>
    let mut v1_iter = v1.iter_mut();

    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);

    // // cannot borrow `v1` as immutable because it is also borrowed as mutable
    // println!("{:?}", v1);
    println!("{:?}", v1_iter);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // fn sum<S>(self) -> S
    // where
    //     Self: Sized,
    //     S: Sum<Self::Item>,
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    println!("{:?}", v1);
    // // borrow of moved value
    // println!("{:?}", v1_iter);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(unused)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

struct Counter {
    count: u32,
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum); // 2*3 + 3*4
}
