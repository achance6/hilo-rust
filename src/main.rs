use std::io;
use rand;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let target = rand::thread_rng().gen_range(1..=100);

    println!("The target number is {target}");

    println!("Please input your guess.");

    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Line should be read successfully");

    println!("You guessed: {}", guess);
}
