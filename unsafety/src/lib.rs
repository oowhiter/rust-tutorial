#![allow(unused)]

mod raw_pointer {
    /// Different from references and smart pointers, raw pointers:
    /// - Are allowed to ignore the borrowing rules by having both immutable
    ///   and mutable pointers or multiple mutable pointers to the same location
    /// - Aren’t guaranteed to point to valid memory
    /// - Are allowed to be null
    /// - Don’t implement any automatic cleanup
    #[test]
    fn pointer_datatype() {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        println!("{:?}, {:?}", r1, r2);

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }

        let address = 0x012345usize;
        let r = address as *const i32;
        println!("{:?}", r);
    }
}

mod unsafe_routine {
    unsafe fn dangerous() {}

    #[test]
    fn call_dangerous() {
        unsafe {
            dangerous();
        }
    }

    // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = slice.len();
    //     assert!(mid <= len);
    //     (&mut slice[..mid], &mut slice[mid..])
    // }

    #[test]
    fn define_unsafe_func() {
        use std::slice;
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
                )
            }
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];

        let (a, b) = split_at_mut(&mut v[..], 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
}

mod use_extern {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[test]
    fn use_extern() {
        unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }
    }
}

mod use_static {
    static HELLO_WORLD: &str = "Hello, world!";

    #[test]
    fn use_static() {
        println!("name is: {}", HELLO_WORLD);
    }

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    #[test]
    fn use_static_mut() {
        add_to_count(3);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}

mod unsafe_trait {
    unsafe trait Foo {
        //
    }

    unsafe impl Foo for i32 {
        //
    }
}
