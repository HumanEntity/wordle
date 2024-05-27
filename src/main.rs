use color_eyre::eyre::Result;
use dialoguer::Input;
use wordle::{dict::Dict, word::Word};

fn main() -> Result<()> {
    color_eyre::install()?;

    let dict = Dict::from_file("dict.txt")?;

    let test_word = Word::try_from("fuzzy")?;
    loop {
        let word: String = Input::new().with_prompt("guess").interact_text().unwrap();
        let word = Word::try_from(word.as_ref())?;
        if !dict.is_valid(&word) {
            continue;
        }
        let matches = test_word.match_word(&word);
        println!("{}", word.format(matches));
        if word == test_word {
            break;
        }
    }
    Ok(())
}
