use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("a play game!");
    let secret_number = rand::thread_rng().gen_range(1..50);
    loop {
        println!("please Guess the number!");

        // println!("The secret number is: {}", secret_number);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read the number!");
        // println!("you guess the number is {}", guess)

        let guess: i32 = guess.trim().parse().expect("parse the string to int failed");


        println!("You guess number {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("to small"),
            Ordering::Greater => println!("to large"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
    
    println!("gameover")

}
