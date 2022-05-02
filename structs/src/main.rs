fn main() {
    // 5-1
    define_struct();
    define_struct_build_func();
    use_field_init_shorthand();
    instantiate_with_struct_update_syntax();
    use_tuple_structs();
    define_unit_like_structs();
    struct_requires_ownership_or_lifetime();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn define_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

fn define_struct_build_func() {
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}

fn use_field_init_shorthand() {
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}

fn instantiate_with_struct_update_syntax() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
}

fn use_tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn define_unit_like_structs() {
    struct Unit();
    let _ = Unit();
}

fn struct_requires_ownership_or_lifetime() {
    // // not compilable, use String
    // struct User {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool,
    // }

    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };
}
