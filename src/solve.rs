use crate::thread_pool::*;
use crate::wordle::round;
use crate::wordle::Color;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread::available_parallelism;

fn index<T>(v: &Vec<T>, item: &T) -> Option<usize>
where
    T: PartialEq,
{
    for (i, elem) in v.iter().enumerate() {
        if *elem == *item {
            return Some(i);
        }
    }
    None
}

pub fn update_possibilities(
    possibilities: &mut [Vec<char>; 5],
    word: &str,
    colors: &[Color; 5],
    green: &mut HashMap<char, Vec<usize>>,
    yellow: &mut Vec<char>,
) {
    for (i, letter) in word.chars().enumerate() {
        match colors[i] {
            Color::Green => {
                possibilities[i] = vec![];
                possibilities[i].push(letter);
                green
                    .entry(letter)
                    .and_modify(|s| {
                        s.push(i);
                    })
                    .or_insert_with(|| vec![i]);
            }
            Color::Yellow => {
                if let Some(index) = index(&possibilities[i], &letter) {
                    possibilities[i].swap_remove(index);
                }
                yellow.push(letter);
            }
            Color::Black => {}
        }
    }

    for (i, letter) in word.chars().enumerate() {
        if let Color::Black = colors[i] {
            if yellow.contains(&letter) {
                if let Some(index) = index(&possibilities[i], &letter) {
                    possibilities[i].swap_remove(index);
                }
            } else if let Some(s) = green.get(&letter) {
                for j in 0..5 {
                    if !s.contains(&j) {
                        if let Some(index) = index(&possibilities[j], &letter) {
                            possibilities[j].swap_remove(index);
                        }
                    }
                }
            } else {
                for j in 0..5 {
                    if let Some(index) = index(&possibilities[j], &letter) {
                        possibilities[j].swap_remove(index);
                    }
                }
            }
        }
    }
}

fn is_possible_word(possibilities: &[Vec<char>; 5], word: &str, yellow: &Vec<char>) -> bool {
    let mut yellow_in_word: Vec<char> = vec![];
    for (i, letter) in word.chars().enumerate() {
        if !possibilities[i].contains(&letter) {
            return false;
        }
        if yellow.contains(&letter) {
            yellow_in_word.push(letter);
        }
    }
    for i in yellow {
        if !yellow_in_word.contains(i) {
            return false;
        }
    }
    return true;
}

pub fn update_possible_words<'a>(
    possibilities: &[Vec<char>; 5],
    possible_words: Vec<String>,
    yellow: &Vec<char>,
) -> Vec<String> {
    possible_words
        .into_iter()
        .filter(|word| is_possible_word(possibilities, &word, yellow))
        .collect()
}

pub fn optimal_word(
    possibilities: &[Vec<char>; 5],
    possible_words: Vec<String>,
    green: &HashMap<char, Vec<usize>>,
    yellow: &Vec<char>,
    progress: bool,
) -> String {
    let mut min_word = String::from("");
    let mut min_average_length: f64 = f64::MAX;
    let mut recievers = vec![];
    let parellelisms = available_parallelism().unwrap().get();
    //println!("{}", parellelisms);
    let pool = ThreadPool::new(parellelisms);
    for simulated_guessed_word in possible_words.clone() {
        let new_possibilities = possibilities.clone();
        let new_possible_words = possible_words.clone();
        let new_green = green.clone();
        let new_yellow = yellow.clone();
        let (tx, rx) = mpsc::channel();
        recievers.push((simulated_guessed_word.clone(), rx));
        pool.execute(move || {
            let mut total_length = 0;
            for simulated_actual_word in new_possible_words.clone() {
                let mut new_possible_words = new_possible_words.clone();
                round(
                    &simulated_guessed_word,
                    Some(&simulated_actual_word),
                    &mut new_possibilities.clone(),
                    &mut new_possible_words,
                    &mut new_green.clone(),
                    &mut new_yellow.clone(),
                    false,
                    false,
                    false,
                    false,
                );
                total_length += new_possible_words.len();
            }
            tx.send((total_length as f64) / (new_possible_words.len() as f64))
                .unwrap();
        });
    }
    let progress_bar = if progress {
        Some(ProgressBar::new(possible_words.len() as u64))
    } else {
        None
    };
    for (simulated_guessed_word, rx) in recievers {
        let average_length = rx.recv().unwrap();
        if average_length < min_average_length {
            min_word = simulated_guessed_word;
            min_average_length = average_length;
        }
        if let Some(ref progress_bar) = progress_bar {
            progress_bar.inc(1);
        }
    }
    if let Some(progress_bar) = progress_bar {
        progress_bar.finish();
    }
    String::from(min_word)
}
