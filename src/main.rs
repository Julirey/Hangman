//! Hangman is a command-line game where players guess a hidden word.
//! Users will be able to input a specific letter that will be compared
//! to the letters of the hidden word, if the letter is in the word then their respective 
//! positions will be revealed. Once all letters are revelaed, the player wins and the game ends.

use std::io;
use rand::prelude::IndexedRandom;

/// Main function for the Hangman game
/// 
/// # Arguments
/// * None.
/// 
/// # Returns
/// * Nothing.
fn main() {
    // List of words for the game
    let words_list = vec![
        "round".to_string(),
        "dark".to_string(),
        "white".to_string(),
        "playful".to_string(),
        "giraffe".to_string(),
        "sunflower".to_string(),
    ];

    // Get random word from list
    let random_word = get_random_word(&words_list);

    // Setup player data
    let guess_chars = vec![];
    let guess_vec: Vec<char> = random_word.clone().chars().collect();
    let output_string_vec = vec!['_'; guess_vec.len()];

    let mut player_0 = PlayerRoot::new(
        &random_word,
        0,
        output_string_vec,
        20,
        guess_chars,
    );

    // Begin game
    println!("Welcome to the Rust Hangman Game!, enter a letter to begin:");
    println!(
        "Fill in the blank spaces{:?} | Max attempts {:?}",
        player_0.output_string, player_0.max_attempts
    );

    // Loop every time the player makes a guess until the game is over
    loop {
        // Save the input from the user
        let mut guess = String::from("");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let altered_guess: char = match guess.trim().chars().next() {
            Some(val) => val,
            _ => {
                println!("No letter inputted. Please type a letter");
                '0'
            }
        };

        // Checks if guess is valid
        if !player_0.output_string.contains(&altered_guess) {

            // Add a guess to the counter
            player_0.guesses += 1;

            // Calculate remaining guesses 
            let guess_score = player_0.max_attempts - player_0.guesses;

            // if the guess is incorrect, prompts the player with failure message
            if !player_0.word.contains(altered_guess) {
                println!(
                    "Wrong guess\nFill in the blank spaces{:?} | Guesses remaining: {:?}",
                    player_0.output_string, guess_score
                );
            }

            // if the guess is correct, adds the char to the correct guesses list
            for n in player_0.word.char_indices() {
                if n.1 == altered_guess {

                    player_0.correct_guesses.push(n.1);
                    player_0.output_string[n.0] = n.1;

                    println!(
                        "Fill in the blank spaces{:?} | Guesses remaining: {:?}",
                        player_0.output_string, guess_score
                    );
                }
            }
        } else {
            //
            println!("That letter is already revealed. Guess again")
        }

        // If the player wins
        if player_0.correct_guesses.len() == guess_vec.len() {
            println!("You Win");
            break;
        }

        // If the player loses
        if player_0.max_attempts == player_0.guesses {
            println!("Game Over \n The word is {:?}", player_0.word);
            break;
        }
    }
}

/// Class that holds player data and game state
struct PlayerRoot {
    word: String,
    guesses: i8,
    output_string: Vec<char>,
    max_attempts: i8,
    correct_guesses: Vec<char>,
}

impl PlayerRoot {
    fn new(
        word: &str,
        guesses: i8,
        output_string: Vec<char>,
        max_attempts: i8,
        correct_guesses: Vec<char>,
    ) -> PlayerRoot {
        PlayerRoot {
            word: String::from(word),
            guesses,
            output_string,
            max_attempts,
            correct_guesses,
        }
    }
}

/// Selects a random string from a list and returns it
/// 
/// # Arguments
/// * `list` - an array of strings.
/// 
/// # Returns
/// * A single string from the list
fn get_random_word(list: &Vec<String>) -> String {
    let word = list.choose(&mut rand::rng()).unwrap();
    word.to_string()
}
