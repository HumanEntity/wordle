use std::collections::HashMap;

use color_eyre::eyre::Result;
use dialoguer::Input;
use wordle::{
    dict::Dict,
    word::{LetterMatch, Word},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let dict = Dict::from_file("dict.txt")?;

    let mut guessed = Vec::new();

    let mut found_letters = HashMap::new();

    let hidden = Dict::from_file("dict_common.txt")?.get_random();
    loop {
        let word: String = Input::new().with_prompt("guess").interact_text().unwrap();
        let word = Word::try_from(word.as_ref())?;
        if !dict.is_valid(&word) {
            continue;
        }
        let matches = hidden.match_word(&word);
        guessed.push((word, matches));

        for (c, m) in word.0.iter().copied().zip(matches) {
            let found = found_letters
                .get(&c)
                .copied()
                .unwrap_or(LetterMatch::Absent);
            match found {
                LetterMatch::Correct => (),
                _ => {
                    found_letters.insert(c, m);
                }
            }
        }

        let mut found = found_letters.iter().collect::<Vec<_>>();
        found.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
        for (k, v) in &found {
            print!("{}", v.format_char(**k));
        }
        println!();

        // println!("{found_letters:?}");

        for (w, m) in &guessed {
            println!("{}", w.format(*m));
        }

        if word == hidden {
            break;
        }
    }
    Ok(())
}
