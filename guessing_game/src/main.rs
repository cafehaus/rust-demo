use std::io;
use std::cmp::Ordering;
use rand:Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小了");
        Ordering::Greater => println!("太大了");
        Ordering::Equal => {
            println!("猜对了");
            break;
        }
    }
    println!("The secret number is: {secret_number}");
}
