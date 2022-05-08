fn main() {
    println!("tuple study!");

    // 相同数据类型的tuple
    let tup = (1,2,4);
    println!("相同数据类型的tuple");
    // println!(tup);

    println!("不同数据类型的tuple");
    let tup: (i32, bool, u8) = (1000, false, 10);
    // println!( tup);

    println!("访问tup元素");
    println!("first: {}",tup.0);
    println!("second: {}",tup.1);

    // tuple解构
    let (x,y,z) = tup;
    println!("tuple解构 {},{},{}",x,y,z);

}