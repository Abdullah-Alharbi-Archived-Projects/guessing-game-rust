use std::io;

fn main() {
    println!("Guessing game!");
    println!("please select the game you want to play!");
    println!("1- Guess the number\n2- Guess the word");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess is: {}", guess)
}
