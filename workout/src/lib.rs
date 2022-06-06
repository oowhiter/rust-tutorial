use std::cmp;
use std::collections::HashMap;
use std::hash;
use std::thread;
use std::time::Duration;

struct Cacher<K, V, T>
where
    K: hash::Hash + cmp::Eq + Copy,
    V: Copy,
    T: Fn(K) -> V,
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<K, V, T> Cacher<K, V, T>
where
    K: hash::Hash + cmp::Eq + Copy,
    V: Copy,
    T: Fn(K) -> V,
{
    fn new(calculation: T) -> Cacher<K, V, T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: K) -> V {
        let value = self.value.get(&arg);
        match value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[allow(unused)]
mod version3 {
    use std::thread;
    use std::time::Duration;

    pub fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure(intensity));
            println!("Next, do {} situps!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_closure(intensity));
            }
        }
    }
}

#[allow(unused)]
mod version2 {
    use std::thread;
    use std::time::Duration;

    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    pub fn generate_workout(intensity: u32, random_number: u32) {
        let expensive_result = simulated_expensive_calculation(intensity);

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result);
            println!("Next, do {} situps!", expensive_result);
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_result);
            }
        }
    }
}

#[allow(unused)]
mod version1 {
    use std::thread;
    use std::time::Duration;

    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    pub fn generate_workout(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps!",
                simulated_expensive_calculation(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    simulated_expensive_calculation(intensity)
                );
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _ = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn call_with_different_values_str() {
    let mut c = Cacher::new(|a| a);

    let _ = c.value("foo");
    let v2 = c.value("bar");

    assert_eq!(v2, "bar");
}

#[test]
fn closure_traits() {
    let x = 4;

    let equal_to_x = |z| z == x; // FnOnce + FnMut + Fn

    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
fn closure_move() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x; // FnOnce?

    // // cannot use x anymore
    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
