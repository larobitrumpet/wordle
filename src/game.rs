use crate::user_io::get_word;
use crate::wordle::*;
use fastrand;

pub fn run() {
    let (dictionary, mut possible_words, mut possibilities, mut green, mut yellow) = setup();
    let i = fastrand::usize(..dictionary.len());
    let word: &str = &dictionary[i];
    //println!("{}", word);

    // Uncomment to compute the optimal first word
    //println!("Optimal word: {}", optimal_word(&possibilities, possible_words.clone(), &green, &yellow));
    // The optimal word was precomputed using the preceading function
    println!("Optimal word: lares");
    let mut guessed_correctly = false;
    for _ in 0..6 {
        let word_guessed = get_word(&dictionary);
        guessed_correctly = round(
            &word_guessed,
            Some(word),
            &mut possibilities,
            &mut possible_words,
            &mut green,
            &mut yellow,
            true,
            true,
            true,
            true,
        );
        if guessed_correctly {
            break;
        }
    }
    if !guessed_correctly {
        println!("{word}");
    }
}
