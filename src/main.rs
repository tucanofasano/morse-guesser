
use std::io;
use rand::seq::IteratorRandom;

use crate::enums::GuessMode;

mod morse;
mod enums;

fn main() {
    let map = morse::morse_map();
    let guess_modes = [GuessMode::Code, GuessMode::Letter];

    let mut rng = rand::rng();

    println!("🎯 Welcome to Guess the Number!");

    loop {
        let (&letter, &code) = map.iter().choose(&mut rng).unwrap();
        let guess_mode = *guess_modes.iter().choose(&mut rng).unwrap();

        let from_guess: String;
        let to_guess: String;

        match guess_mode {
            GuessMode::Code => {
                from_guess = code.to_string();
                to_guess = letter.to_string();
            },
            GuessMode::Letter => {
                from_guess = letter.to_string();
                to_guess = code.to_string();
            }
        };

        println!("{from_guess} :");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim().to_uppercase();

        if guess.eq(&to_guess) {
            println!("Correct!\n");
        }
        else {
            println!("Wrong! The correct answer was {to_guess}\n");
        }
    }
}
