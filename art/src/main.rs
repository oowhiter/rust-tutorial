use art;

// use art::kinds::PrimaryColor;
// use art::utils::mix;

use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let mixed = mix(red, yellow);
    println!("{:?}", mixed);
}
