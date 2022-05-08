fn main() {
    println!("Hello, world!");

    // 简单的函数调用
    call_func();

    // 表达式是有返回值，一个区块最后一个方法不加;表示是一个表达式
    // 语句没有返回值，或者说返回值其实是一个空tuple ()
    let x = {
        let x = 3;
        // 这里 x+1 没有; 结束，它表示的是一个表达式，并不是一个语句，语句需要;结尾
        x + 1
    };
    println!("x value is {}",x);

    // 接收一个表达式
    let z = express_int();
    println!("z value is {}",z);

    // 语句没有返回值，强制使用return
    let s = return_func();
    println!("z value is {}",s);
}


fn call_func() {
    println!("call_func")
}

fn express_int() -> i32 {
    let x = 5 + 1;
    // 没有加;它是一个表达式，不是语句，返回值是这个表达式
    x
}

fn return_func() -> i8 {
    return 10;
}
//
// fn hello() -> &str {
//     "hello"
// }
//
// fn world() -> &str {
//     return "world";
// }