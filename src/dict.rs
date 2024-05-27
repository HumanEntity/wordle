use std::{collections::HashSet, fs::File, io::Read, path::Path};

use crate::word::{Word, WordError};

#[derive(Debug, thiserror::Error)]
pub enum DictError {
    #[error("Io error")]
    Io(#[from] std::io::Error),
    #[error("Word error")]
    Word(#[from] WordError),
}
type Result<T> = core::result::Result<T, DictError>;

#[derive(Debug, Default, Clone)]
pub struct Dict {
    valid_words: HashSet<u64>,
}

impl Dict {
    #[must_use]
    pub fn new() -> Self {
        Self {
            valid_words: HashSet::new(),
        }
    }

    pub fn add_word(&mut self, word: &Word) {
        self.valid_words.insert(word.hash());
    }

    pub fn add_words(&mut self, words: &[Word]) {
        self.valid_words
            .extend(words.iter().map(Word::hash).collect::<Vec<_>>())
    }

    pub fn is_valid(&self, word: &Word) -> bool {
        self.valid_words.contains(&word.hash())
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Self::parse_str(buffer)
    }

    pub fn parse_str(str: impl AsRef<str>) -> Result<Self> {
        Ok(Self::from(
            str.as_ref()
                .split_whitespace()
                .map(Word::try_from)
                .map(|x| x.map(|x| x.hash()))
                .map(|x| x.map_err(DictError::from))
                .collect::<Result<HashSet<_>>>()?,
        ))
    }
}

impl From<HashSet<u64>> for Dict {
    fn from(value: HashSet<u64>) -> Self {
        Self { valid_words: value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use color_eyre::eyre::Result;

    #[test]
    fn parse_str() -> Result<()> {
        let dict = Dict::parse_str("hello world")?;
        assert!(dict.is_valid(&Word::try_from("hello")?));
        assert!(dict.is_valid(&Word::try_from("world")?));
        assert!(!dict.is_valid(&Word::try_from("fuzzy")?));

        Ok(())
    }
}
