use std::io::{stdin, stdout, Write};
use crate::wordle::Color;

// Gets a string input from the user
fn get_input(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut line).unwrap();
    line.pop();
    if line.len() == 0 {
        return line;
    }
    if &line[(line.len() - 1)..] == "\r" {
        line.pop();
    }
    line
}

// Validate that `word` is a 5 letter word
fn validate_word(word: &str) -> bool {
    if word.len() != 5 {
        return false;
    }
    word.chars().all(char::is_alphabetic)
}

// Get a word from the user. Keeps prompting the user
// until they enter a valid 5 letter word in the dictionary
pub fn get_word(dictionary: &Vec<String>) -> String {
    loop {
        let word = get_input("Word: ");
        if !validate_word(&word) {
            println!("That is not a valid 5 letter word");
            continue;
        }
        if !dictionary.contains(&word) {
            println!("That word is not in the dictionary");
            continue;
        }
        return word;
    }
}

pub fn print_word(word: &str, colors: &[Color; 5]) {
    for (i, letter) in word.chars().enumerate() {
        match colors[i] {
            Color::Green  => print!("\x1B[0;42m"),
            Color::Yellow => print!("\x1B[0;43m"),
            Color::Black  => print!("\x1B[0;40m"),
        }
        print!("{}\x1B[0m", letter);
    }
    println!();
}

pub fn get_user_colors() -> [Color; 5] {
    let mut colors: [Color; 5] = [Color::Black; 5];
    'input: loop {
        let col = get_input("Colors: ");
        if col.len() != 5 {
            println!("Please enter 5 letters");
            continue;
        }
        for (i, c) in col.chars().enumerate() {
            match c {
                'g' => colors[i] = Color::Green,
                'G' => colors[i] = Color::Green,
                'y' => colors[i] = Color::Yellow,
                'Y' => colors[i] = Color::Yellow,
                'b' => colors[i] = Color::Black,
                'B' => colors[i] = Color::Black,
                _ => {
                    println!("That is not a valid color sequence");
                    continue 'input;
                },
            }
        }
        break;
    }
    colors
}
