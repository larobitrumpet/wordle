use crate::solve::*;
use crate::user_io::*;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Green,
    Yellow,
    Black,
}

pub fn get_colors(word: &str, word_guessed: &str) -> [Color; 5] {
    // Create a hashmap for each letter in the word that contains
    // the number of times the letter appears in the word and a
    // hashset of the indexes where those letters appear.
    let mut letters: HashMap<char, (u8, Vec<usize>)> = HashMap::new();
    for (i, letter) in word.chars().enumerate() {
        letters
            .entry(letter)
            .and_modify(|(n, s)| {
                *n += 1;
                s.push(i);
            })
            .or_insert_with(|| {
                let s = vec![i];
                (1, s)
            });
    }

    let mut colors: [Color; 5] = [Color::Black; 5];

    // Determine the appropriate color for each letter and
    // print out the coresponding ANSI escape sequence

    // First, figure out all of the green letters.
    // This must be done first before figuring out other
    // colors to ensure that the correct number of green
    // and yellow letters are printed if the letter
    // appears multiple times.
    for (i, letter) in word_guessed.chars().enumerate() {
        if let Some((n, s)) = letters.get_mut(&letter) {
            // `n` helps us keep track of how many times a
            // letter appears. This will allow us to print
            // the correct number of yellow letters later on.
            if s.contains(&i) {
                *n -= 1;
                colors[i] = Color::Green;
            }
        }
    }

    // Then, we can figure out the yellow and black letters.
    for (i, letter) in word_guessed.chars().enumerate() {
        if let Some((n, _)) = letters.get_mut(&letter) {
            if colors[i] == Color::Green {
                continue;
            }
            // If `n` is 0, we have printed green and yellow
            // for that letter the exact number of times it
            // appears in the word, so every other time it
            // appears it should be black.
            if *n > 0 {
                *n -= 1;
                colors[i] = Color::Yellow;
            } else {
                colors[i] = Color::Black;
            }
        } else {
            colors[i] = Color::Black;
        }
    }

    colors
}

pub fn setup() -> (
    Vec<String>,
    Vec<String>,
    [Vec<char>; 5],
    HashMap<char, Vec<usize>>,
    Vec<char>,
) {
    let contents = fs::read_to_string("allowed_words.txt").unwrap();
    let dictionary: Vec<String> = contents.lines().map(|line| String::from(line)).collect();
    let possible_words = dictionary.clone();
    let mut possibilities: [Vec<char>; 5] = [vec![], vec![], vec![], vec![], vec![]];
    for i in &mut possibilities {
        for c in 'a'..='z' {
            i.push(c);
        }
    }
    let green: HashMap<char, Vec<usize>> = HashMap::new();
    let yellow: Vec<char> = vec![];
    (dictionary, possible_words, possibilities, green, yellow)
}

pub fn round(
    word_guessed: &str,
    word: Option<&str>,
    possibilities: &mut [Vec<char>; 5],
    possible_words: &mut Vec<String>,
    green: &mut HashMap<char, Vec<usize>>,
    yellow: &mut Vec<char>,
    debug: bool,
    show_optimal: bool,
    print_colored_word: bool,
    optimal_word_progress: bool,
) -> bool {
    let colors = if let Some(word) = word {
        let colors = get_colors(word, word_guessed);
        if print_colored_word {
            print_word(word_guessed, &colors);
        }
        colors
    } else {
        get_user_colors()
    };
    update_possibilities(possibilities, word_guessed, &colors, green, yellow);
    *possible_words = update_possible_words(&possibilities, possible_words.clone(), &yellow);
    if debug {
        println!("{:?}", possible_words);
        println!("{:?}", possibilities);
    }
    if let Some(word) = word {
        if word.eq(word_guessed) {
            return true;
        }
    } else {
        print_word(&word_guessed, &colors);
    };
    if show_optimal {
        println!(
            "Optimal word: {}",
            optimal_word(
                &possibilities,
                possible_words.clone(),
                &green,
                &yellow,
                optimal_word_progress
            )
        );
    }
    false
}
