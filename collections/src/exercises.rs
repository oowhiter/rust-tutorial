use std::collections::HashMap;

pub fn calc(numbers: &Vec<i32>) {
    let mut sum = 0;
    let mut mode_map: HashMap<i32, u32> = HashMap::new();

    for item in numbers {
        sum += *item;
        let val = mode_map.entry(*item).or_default();
        *val += 1;
    }

    let len = numbers.len();

    let mean = sum as f64 / len as f64;

    let mut numbers = numbers.clone();
    numbers.sort();
    let medium = numbers.get(len / 2).unwrap();

    let mut mode = 0;
    let mut max_occurs = u32::MIN;
    for (k, v) in &mode_map {
        if *v > max_occurs {
            max_occurs = *v;
            mode = *k;
        }
    }

    println!("mean: {}", mean);
    println!("medium: {}", medium);
    println!("mode: {}", mode);
}

pub fn pig_latin(s: &str) -> String{
    let vowels = vec!['a', 'i', 'u', 'e', 'o'];
    let first_c =  s.chars().next().unwrap();
    if vowels.contains(&first_c) {
        format!("{}-hay", &s)
    } else {
        format!("{}-{}ay", &s[1..], first_c)
    }
}
