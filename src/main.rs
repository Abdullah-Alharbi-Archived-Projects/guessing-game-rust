use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game!");
    println!("please select the game you want to play!");
    println!("1- Guess the number\n2- Guess the word");

    loop {
        let mut game = String::new();

        io::stdin()
            .read_line(&mut game)
            .expect("Failed to read line");

        let game: u32 = match game.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if game == 1 {
            number();
            break;
        } else {
            word();
            break;
        }
    }
}

fn number() {
    println!("Guess the number started enter a number between 1 and 100 ...");
    // generate random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}

fn word() {
    println!("not implemented yet.");
}
