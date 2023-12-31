use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("the secret number is {}", secret_number);

    println!("PLease input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to readline");

    println!("You guessed: {}", guess)

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!");
        Ordering::Greater => println("Too big!");
        Ordering::Equal => println!("You win!");
    };

}
