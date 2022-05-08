fn main() {
    // loop 表达式
    println!("loop express!");

    let mut counter: i32 = 1;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break 返回 counter * 2 表达式结果
            break counter * 2;
        }
    };

    println!("result value is {}", result);

    // for 表达式
    let arr:[i32; 5] = [1, 2, 3, 4, 5];

    for element in arr.iter() {
        println!("element value is {}", element)
    }
}