use rand::Rng;
use std::{cmp::Ordering, io};
// #![allow(unused)]
// #![allow(unused_imports)]
// use std::io::prelude::*;
fn main() {
    println!("Welcom To Guess The Number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed:{guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small :("),
        Ordering::Greater => println!("Damm that is too big"),
        Ordering::Equal => {
            println!("You get it right!")
        }
    }
}
