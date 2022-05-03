mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod pub_struct {
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            // non-pub by default
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        // meal.seasonal_fruit = String::from("blueberries");
    }
}

mod pub_enum {
    mod back_of_house {
        pub enum Appetizer {
            // pub by default
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}

use crate::front_of_house::hosting;

pub fn short_due_to_use() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

mod use_self {
    use self::super::front_of_house::hosting;

    fn f() {
        hosting::add_to_waitlist();
    }
}

// Not recommended. Where is the code which defines add_to_waitlist?
use crate::front_of_house::hosting::add_to_waitlist;

// In convention, struct, enum, etc. are specified with full-path.
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// // Use parent mod if an element name is duplicated.
// use std::fmt::Result;
// use std::io::Result;
use std::fmt;
use std::io;

// Use `as` for alias
use std::fmt::Result;
use std::io::Result as IoResult;

mod pub_use {
    pub use crate::front_of_house::hosting;
}

fn pub_use_f() {
    pub_use::hosting::add_to_waitlist();
}

mod arrange_use {
    use std::{cmp::Ordering, io};
    // use std::cmp::Ordering;
    // use std::io;
}

mod use_mod_self {
    use std::io::{self, Write};
    // use std::io;
    // use std::io::Write;
}

mod use_glob_operator {
    // Be careful! Often used for testing.
    use std::collections::*;
}
