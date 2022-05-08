fn main() {
    let s = String::from("abc");

    let (s1, s1_len) = get_length(s);

    // 此时s不可用了，因为s被 move 到s1
    //println!("s value is", s);
    println!("s1 value is:{}, s1 len is {}", s1, s1_len);

    // 借用
    let s_borrow = String::from("def");

    // 因为s是借用给函数，所有只给出指针，不允许函数内部修改值
    let s_borrow_len = get_length_borrow(&s_borrow);
    println!("s value is: {}, s borrow len is: {}", s_borrow, s_borrow_len);
}

fn get_length_borrow(str: &String) -> usize {
    str.len()
}

fn get_length(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}