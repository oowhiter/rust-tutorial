#[macro_export]
macro_rules! myvec {
    ( $($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[test]
fn test() {
    let v = myvec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
}
