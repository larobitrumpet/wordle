use crate::solve::*;
use crate::user_io::get_user_colors;
use std::collections::HashMap;
use std::fs;

pub fn play() {
    let contents = fs::read_to_string("allowed_words.txt").unwrap();
    let dictionary: Vec<String> = contents.lines().map(|line| String::from(line)).collect();

    let mut possible_words = dictionary.clone();
    let mut possibilities: [Vec<char>; 5] = [vec![], vec![], vec![], vec![], vec![]];
    for i in &mut possibilities {
        for c in 'a'..='z' {
            i.push(c);
        }
    }
    let mut green: HashMap<char, Vec<usize>> = HashMap::new();
    let mut yellow: Vec<char> = vec![];

    // Uncomment to compute the optimal first word
    //println!("Optimal word: {}", optimal_word(&possibilities, possible_words.clone(), &green, &yellow));
    // The optimal word was precomputed using the preceading function
    println!("Optimal word: lares");
    let mut word_guessed = String::from("lares");
    for _ in 0..6 {
        //let word_guessed = get_word(&dictionary);
        let colors = get_user_colors();
        update_possibilities(
            &mut possibilities,
            &word_guessed,
            &colors,
            &mut green,
            &mut yellow,
        );
        possible_words = update_possible_words(&possibilities, possible_words, &yellow);
        println!("{:?}", possible_words);
        println!("{:?}", possibilities);
        word_guessed = optimal_word(
            &possibilities,
            possible_words.clone(),
            &green,
            &yellow,
            true,
        );
        println!("Optimal word: {}", word_guessed);
        if possible_words.len() <= 1 {
            break;
        }
    }
}
