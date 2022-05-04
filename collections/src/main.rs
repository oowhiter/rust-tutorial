mod exercises;
mod hash_maps;
mod strings;
mod vectors;

fn main() {
    // 8-1
    vectors::make_vec1();
    let mut v = vectors::make_vec2();
    vectors::update_vec1(&mut v);
    println!("{:?}", v);
    vectors::scope_drop();
    vectors::read_vec();
    vectors::iter_vec();
    vectors::heterogeneous_vec();
    // 8-2
    strings::make_string();
    strings::make_string_from_str();
    strings::make_string_encoded();
    strings::push_string();
    strings::get_part_of_string();
    // 8-3
    hash_maps::make_hash_map();
    hash_maps::make_hash_map_with_collect();
    hash_maps::get_hash_map_value();
    hash_maps::update_hash_map();

    exercises::calc(&vec![1, 3, 4, 3, 2, 1, 1, 5]);

    let s = "egoh";
    println!("{} -> {}", s, exercises::pig_latin(s));
    let s = "fuga";
    println!("{} -> {}", s, exercises::pig_latin(s));
}
