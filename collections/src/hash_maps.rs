use std::collections::HashMap;

pub fn make_hash_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Key and value are moved.
    let field_name = String::from("Green");
    scores.insert(field_name, 100);
    // println!(field_name)

    scores
}

pub fn make_hash_map_with_collect() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

pub fn get_hash_map_value() {
    let scores = make_hash_map();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    assert_eq!(score, Some(&10));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

pub fn update_hash_map() {
    let mut scores = make_hash_map();

    // insert
    assert_eq!(scores.get(&String::from("Blue")), Some(&10));
    scores.insert(String::from("Blue"), 25);
    assert_eq!(scores.get(&String::from("Blue")), Some(&25));

    // or_insert
    scores.entry(String::from("Blue")).or_insert(1);
    assert_eq!(scores.get(&String::from("Blue")), Some(&25));

    assert_eq!(scores.get(&String::from("Red")), None);
    scores.entry(String::from("Red")).or_insert(1);
    assert_eq!(scores.get(&String::from("Red")), Some(&1));

    //
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
