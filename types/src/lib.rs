#![allow(unused)]

mod type_alias {
    type Kilometers = i32;

    #[test]
    fn use_type_alias() {
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
        assert_eq!(x, y);
    }

    type Thunk = Box<dyn Fn() + Send + 'static>;

    #[test]
    fn use_type_alias1() {
        let f: Thunk = Box::new(|| println!("hi"));
    }

    use std::fmt;
    use std::io::Error;
    use std::result;

    type Result<T> = result::Result<T, std::io::Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
}

mod never {
    fn foo() -> ! {
        panic!("never return")
    }
    fn bar() -> ! {
        loop {
            //
        }
    }
}

mod dst {
    fn dst() {
        // let s1: str = "Hello there!";
        // let s2: str = "How's it going?";
    }

    // use &T, not T for args
    fn sized<T: ?Sized>(t: &T) {
        //
    }
}
