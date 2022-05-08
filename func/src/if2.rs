fn main() {
    // If 表达式
    println!("If express!");

    // If是一个表达式，所以可以用let来接收值
    let x = if true { 5 } else { 6 };
    println!("x value is {}", x);

    let y = if true {
        10
    } else {
        11
    };
    println!("y value is {}", y);
}