#![allow(unused)]

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing Button.")
    }
}

// This Screen can hold multi-typed components which meet the `Draw` trait bounds.
// The concrete type of components is resolved at runtime: Dynamic Dispatch.
// (In other words, the entity of called method is determined at runtime with some costs.)
mod trait_object {
    use super::*;

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}

// This Screen can hold single-typed components
// The concrete type of components is resolved at compile time: Static Dispatch.
mod generics {
    use super::*;

    pub struct Screen<T>
    where
        T: Draw,
    {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use trait_object::Screen;
    // // cannot use generics pattern
    // use generics::Screen;

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Drawing SelectBox.")
        }
    }

    #[test]
    fn sample_draw() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };
        screen.run();
    }
}

// // Non-object-safe trait cannot make trait object.
// pub struct Illegal {
//     pub components: Vec<Box<dyn Clone>>,
// }
