fn main() {
    // // compile error
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    //
    //
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... ここではもう有効ではない
                        // // compile error
                        // println!("{}", s);

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫

    // reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{}", len);

    fn calculate_length(s: &String) -> usize {
        // sはStringへの参照
        s.len()
    } // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので
      // 何も起こらない

    mutable_reference();

    string_slice();

    let s = String::from("hello world");
    let first = first_word(&s[..]);
    println!("{}", first);

} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
  //

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  //

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

// // 57 |     fn change(some_string: &String) {
// //    |                            ------- help: consider changing this to be a mutable reference: `&mut String`
// // 58 |         some_string.push_str(", world");
// //    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// fn mutable_reference() {
//     fn main() {
//         let s = String::from("hello");
//         change(&s);
//     }
//     fn change(some_string: &String) {
//         some_string.push_str(", world");
//     }
// }
fn mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("mutable_reference: {}", s);
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
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

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
