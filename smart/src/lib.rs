#![allow(unused)]

mod use_box {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn use_list() {
            let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
            println!("{:?}", list);
        }
    }
}

mod use_deref {
    use std::ops::Deref;
    use std::ops::DerefMut;

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn use_box() {
            let b = Box::new(5);
            println!("b = {}", b);
        }

        #[test]
        fn use_deref() {
            let x = 5;
            let y = &x;
            assert_eq!(5, x);
            assert_eq!(5, *y);
        }

        #[test]
        fn use_box_deref() {
            let x = 5;
            let mut y = Box::new(x);
            assert_eq!(5, x);
            assert_eq!(5, *y);

            *y = 6;
            assert_eq!(5, x);
            assert_eq!(6, *y);
        }

        #[test]
        fn use_mybox_deref() {
            let x = 5;
            let mut y = MyBox::new(x);
            assert_eq!(5, x);
            assert_eq!(5, *y);

            *y = 6;
            assert_eq!(5, x);
            assert_eq!(6, *y);
        }

        #[test]
        fn see_deref_coercion() {
            let m = MyBox::new(String::from("Rust"));
            // hello(&(*m)[..]);
            hello(&m);
        }
    }
}

mod use_drop {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn see_drop() {
            let c = CustomSmartPointer {
                data: String::from("my stuff"),
            };
            let d = CustomSmartPointer {
                data: String::from("other stuff"),
            };
            println!("CustomSmartPointers created.");
        }

        #[test]
        fn manual_drop() {
            let c = CustomSmartPointer {
                data: String::from("some data"),
            };
            println!("CustomSmartPointer created.");
            drop(c);
            // CustomSmartPointerはmainが終わる前にドロップされた
            println!("CustomSmartPointer dropped before the end of main.");
        }
    }
}

mod use_rc {
    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn use_rc() {
            use List::{Cons, Nil};

            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            println!("count after creating a = {}", Rc::strong_count(&a));

            let b = Cons(3, Rc::clone(&a));
            println!("count after creating b = {}", Rc::strong_count(&a));

            {
                let c = Cons(4, Rc::clone(&a));
                println!("count after creating c = {}", Rc::strong_count(&a));
            }

            println!("count after c goes out of scope = {}", Rc::strong_count(&a));
        }
    }
}

mod use_refcell {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 1.0 {
                self.messenger.send("Error: you are over your quota!");
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::RefCell;

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }
        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }
        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);
            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }

        #[test]
        #[should_panic]
        fn double_borrow_mut() {
            let mock_messenger = MockMessenger::new();
            let mut a = mock_messenger.sent_messages.borrow_mut();
            let mut b = mock_messenger.sent_messages.borrow_mut();
            a.push(String::from("a"));
            b.push(String::from("b"));
        }
    }
}

mod rc_of_refcell {
    use std::cell::RefCell;
    use std::rc::Rc;
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn rc_of_refcell() {
            let value = Rc::new(RefCell::new(5));
            let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
            let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
            let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

            *value.borrow_mut() += 10;

            println!("a after = {:?}", a);
            println!("b after = {:?}", b);
            println!("c after = {:?}", c);
        }
    }
}

mod circular_ref {
    use std::cell::RefCell;
    use std::rc::Rc;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match *self {
                Cons(_, ref item) => Some(item),
                Nil => None,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn circular_ref() {
            let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

            println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
            println!("a next item = {:?}", a.tail());

            let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

            println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
            println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
            println!("b next item = {:?}", b.tail());

            if let Some(link) = a.tail() {
                // let a = Rc::new(Cons(5, RefCell::new(Rc::clone(&b))));
                *link.borrow_mut() = Rc::clone(&b);
            }

            println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
            println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

            // Uncomment the next line to see that we have a cycle;
            // it will overflow the stack
            // println!("a next item = {:?}", a.tail());
        }
    }
}

mod weak_ref {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn weak_ref() {
            let leaf = Rc::new(Node {
                value: 3,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            });

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );

            {
                let branch = Rc::new(Node {
                    value: 5,
                    parent: RefCell::new(Weak::new()),
                    children: RefCell::new(vec![Rc::clone(&leaf)]),
                });
                *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

                println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
                println!(
                    // branchのstrong_count = {}, weak_count = {}
                    "branch strong = {}, weak = {}",
                    Rc::strong_count(&branch),
                    Rc::weak_count(&branch),
                );
                println!(
                    "leaf strong = {}, weak = {}",
                    Rc::strong_count(&leaf),
                    Rc::weak_count(&leaf),
                );
            }
            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }
    }
}
