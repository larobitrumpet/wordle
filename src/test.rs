use std::fs;
use std::collections::HashMap;
use indicatif::ProgressBar;
use crate::wordle::round;
use crate::solve::optimal_word;

pub fn test(word: &str, dictionary: &Vec<String>) -> u8 {
    let mut possible_words = dictionary.clone();
    let mut possibilities: [Vec<char>; 5] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ];
    for i in &mut possibilities {
        for c in 'a'..='z' {
            i.push(c);
        }
    }
    let mut green: HashMap<char, Vec<usize>> = HashMap::new();
    let mut yellow: Vec<char> = vec![];

    let mut i: u8 = 1;
    let mut word_guessed = String::from("lares");
    let mut guessed_correctly = round(&word_guessed, Some(word), &mut possibilities, &mut possible_words, &mut green, &mut yellow, false, false, false, false);
    while !guessed_correctly {
        i += 1;
        guessed_correctly = round(&word_guessed, Some(word), &mut possibilities, &mut possible_words, &mut green, &mut yellow, false, false, false, false);
        word_guessed = optimal_word(&possibilities, possible_words.clone(), &green, &yellow, false);
    }

    return i;
}

pub fn run_tests() {
    let contents = fs::read_to_string("allowed_words.txt").unwrap();
    let dictionary: Vec<String> = contents.lines().map(|line| {String::from(line)}).collect();

    let mut total = 0;
    let progress_bar = ProgressBar::new(dictionary.len() as u64);
    for word in dictionary.clone() {
        total += test(&word, &dictionary);
        progress_bar.inc(1);
    }
    progress_bar.finish();
    println!("Average: {}", (total as f64) / (dictionary.len() as f64));
}
