extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1,10);

    println!("Pick a number: ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Input Failed");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number.");

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small."),
        Ordering::Greater => println!("Too large."),
        Ordering::Equal => println!("Correct."),
    }
}
