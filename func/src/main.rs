fn main() {
    println!("struct");

    let normal_struct = build_normal_struct();
    println!("{}", normal_struct.username);
    println!("normal struct {:?}", normal_struct);

    let from_struct = build_struct_from(&normal_struct);
    println!("from struct {:?}", from_struct);

    let tuple_struct = build_tuple_struct();
    println!("tuple struct, {:?}", tuple_struct);

    // impl

    let r1 = Rectangle::new(10, 20);
    let r2 = Rectangle::new(20, 30);
    let r3 = Rectangle::square(5);

    println!("r1 -- r2: {}", r1.can_hold(&r2));
    println!("r1 -- r2: {}", r1.can_hold(&r3));
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }

    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    age: u16,
    boy: bool,
}

// tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);

fn build_normal_struct() -> User {
    User {
        username: String::from("simon"),
        password: String::from("password"),
        age: 10,
        boy: true,
    }
}

fn build_struct_from(user: &User) -> User {
    User {
        username: String::from("hello"),
        password: String::from("pwd"),
        ..*user
    }
}

fn build_tuple_struct() -> Color {
    Color(1, 2, 3)
}