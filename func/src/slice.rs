fn main() {
    let s = String::from("Hello world");
    // or &s[..]
    let bytes = get_slice(&s);
    println!("String object bytes value is: {}", bytes);

    let s1 = "hello world";
    let bytes = get_slice(&s1);
    println!("string bytes value is: {}", bytes)
}

fn get_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}
//
// fn get_slice(s: &String) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i]
//         }
//     }
//
//     &s[..]
// }