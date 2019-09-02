use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

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

    pause();
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
    println!("Guess the word started ...");
    let words = [
        "world", "movie", "discord", "youtube", "twitter", "facebook",
    ];
    let prefix = "_"; // r___ prefix between word or _us_

    // 1. generate random index
    let words_len = words.len();
    let index = rand::thread_rng().gen_range(0, words_len);

    let selected_word = words[index];

    // 2. select 2 or 1 char to show from the selected word
    let selcted_chars = rand::thread_rng().gen_range(1, 3);
    let mut chars: [usize; 2] = [0, 0];

    for i in 0..selcted_chars {
        chars[i] = rand::thread_rng().gen_range(1, selected_word.len())
    }

    // generate the hidden word

    let mut secret_word = String::from("");
    let selected_word_len = selected_word.len() - 1;

    for (i, c) in selected_word.chars().enumerate() {
        if i == chars[0] || i == chars[1] {
            let selected_char = c.to_string();
            secret_word.push_str(&selected_char);
        } else {
            secret_word.push_str(&prefix);
        }

        if i == selected_word_len {
            break;
        }
    }

    println!("Hidden word: {}, {}", secret_word, secret_word.len());

    loop {
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        if word.trim() == selected_word {
            println!("You win!");
            break;
        } else {
            println!("Try again.");
            continue;
        }
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to exit...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
