extern crate rand;

use std::io;
use std::fs;
use rand::Rng;

enum GuessResult {
    Correct,
    Incorrect,
    AlreadyGuessed,
}

fn draw_board(discovered_characters: &str, word_masked: &str, lives: &u8) {
    println!("{} Lives Remaining", lives);
    draw_hangman(lives);
    draw_guesses(discovered_characters);
    println!("Word: {}", word_masked);
}

fn draw_guesses(discovered: &str) {
    print!("Guesses:");
    for (_u, c) in discovered.chars().enumerate() {
        print!(" {}", c);
    }
    println!();
}

fn draw_hangman(lives: &u8) {
    match lives {
        0 => {
            println!("     __  ");
            println!("    |  | ");
            println!("    0  | ");
            println!("   /|\\ | ");
            println!("   / \\ | ");
            println!("      _|_");
        },
        1 => {
            println!("     __  ");
            println!("    |  | ");
            println!("    0  | ");
            println!("   /|\\ | ");
            println!("     \\ | ");
            println!("      _|_");
        },
        2 => {
            println!("     __  ");
            println!("    |  | ");
            println!("    0  | ");
            println!("   /|\\ | ");
            println!("       | ");
            println!("      _|_");
        },
        3 => {
            println!("     __  ");
            println!("    |  | ");
            println!("    0  | ");
            println!("   /|  | ");
            println!("       | ");
            println!("      _|_");
        },
        4 => {
            println!("     __  ");
            println!("    |  | ");
            println!("    0  | ");
            println!("    |  | ");
            println!("       | ");
            println!("      _|_");
        },
        5 => {
            println!("     __  ");
            println!("    |  | ");
            println!("    0  | ");
            println!("       | ");
            println!("       | ");
            println!("      _|_");
        }
        6 => {
            println!("     __  ");
            println!("    |  | ");
            println!("       | ");
            println!("       | ");
            println!("       | ");
            println!("      _|_");
        }
        _ => {
            panic!("Unknown Lives State!");
        }
    }
}

fn get_word(filename: &str) -> String {
    //read dictionary
    let dictionary_string = fs::read_to_string(filename).expect("Could not read file");
    //parse into individual words
    let dictionary = dictionary_string.split(",").collect::<Vec<&str>>();
    //select random word from dictionary
    return dictionary[rand::thread_rng().gen_range(0..dictionary.len())].to_string();
}

fn format_masked_word(word: &str, discovered: &str) -> String {
    let mut result : String = String::new();

    for(_u, c) in word.chars().enumerate() {
        result.push(if discovered.contains(c) {c}
            else {'_'});
    }

    return result;
}

fn read_guess() -> Option<char> {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.trim().chars().nth(0)
}

fn validate_guess(guess: Option<char>) -> bool {
    match guess {
        Some(g) => {
            if !g.is_alphabetic() {false}
            else {true}
        },
        None => {false}
    }
}

fn check_guess(guess: char, discovered_characters: &str, word: &str) -> GuessResult {
    if discovered_characters.contains(guess) {
        return GuessResult::AlreadyGuessed;
    }
    if !word.contains(guess) {
        return GuessResult::Incorrect;
    }

    GuessResult::Correct
}

fn main() {
    //get word from file
    let word: String = get_word("hangman-dictionary.txt");
    //create masked copy of word
    let mut discovered_characters: String = String::new();
    let mut word_masked = format_masked_word(&word, &discovered_characters);
    
    let mut lives: u8 = 6;
    let mut solved: bool = false;

    loop {
        draw_board(&discovered_characters, &word_masked, &lives);
        print!("Enter your guess: ");
        let guess = read_guess();

        if validate_guess(guess) {
            let valid_guess = guess.unwrap().to_lowercase().next().unwrap();

            let guess_result = check_guess(valid_guess, &discovered_characters, &word);

            match guess_result {
                GuessResult::AlreadyGuessed => {
                    println!();
                    println!("You already guessed '{}'", valid_guess);
                },
                GuessResult::Incorrect => {
                    discovered_characters.push(valid_guess);
                    lives -= 1;

                    if lives <= 0 {
                        break;
                    }
                    else {
                        println!();
                        println!("Incorrect! You lost a life!");
                    }
                },
                GuessResult::Correct => {
                    println!();
                    println!("Correct!");
                    discovered_characters.push(valid_guess);
                    word_masked = format_masked_word(&word, &discovered_characters);

                    if !word_masked.contains('_') {
                        solved = true;
                        break;
                    }
                }
            }

        }
        else{
            println!("Please enter a single character (a-z) as your guess.");
        }
    }

    if solved {
        println!("You won in {} guesses!", discovered_characters.len());
    }
    else {
        draw_board(&discovered_characters, &word_masked, &lives);
        println!("Oh no! You lost!");
        println!("The word was {}", word);
    }
    
    
}
