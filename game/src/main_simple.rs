use std::io;

fn main() {
    println!("a play game!");
    println!("please Guess the number!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read the number!");
    println!("you guess the number is {}", guess)
}
