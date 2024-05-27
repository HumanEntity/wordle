use std::{collections::HashMap, mem::transmute};

use owo_colors::OwoColorize;

#[derive(Debug, Clone, thiserror::Error)]
pub enum WordError {
    #[error("Word to small {0}")]
    TooSmall(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterMatch {
    Correct,
    Present,
    Absent,
}
type MatchList = [LetterMatch; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word([char; 5]);

impl Word {
    pub fn match_word(&self, word: &Word) -> [LetterMatch; 5] {
        let mut base_chars = HashMap::new();
        for c in &self.0 {
            let x = base_chars.get(c).copied().unwrap_or(0);
            base_chars.insert(c, x + 1);
        }

        let mut matches = MatchList::absent_all();
        if word == self {
            return LetterMatchList::correct_all();
        }

        // Correctness check
        for (i, (c1, c2)) in self.0.iter().zip(word.0.iter()).enumerate() {
            if c1 == c2 {
                let x = base_chars.get(c1).copied().unwrap_or(0);
                base_chars.insert(c1, x - 1);
                matches[i] = LetterMatch::Correct;
            }
        }

        for (idx, c1) in word.0.iter().enumerate() {
            if matches[idx].is_correct() {
                continue;
            }
            let x = base_chars.get(c1).copied().unwrap_or(0);
            if self.contains(*c1) && x > 0 {
                matches[idx] = LetterMatch::Present;
                base_chars.insert(c1, x - 1);
            }
        }

        matches
    }
    pub fn contains(&self, ch: char) -> bool {
        for c in self.0 {
            if c == ch {
                return true;
            }
        }
        false
    }
    pub fn contains_at(&self, ch: char, idx: usize) -> bool {
        self.0[idx] == ch
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        let a = self.0.map(|x| x as u64);
        (a[0]) | (a[1] << 8) | (a[2] << 16) | (a[3] << 24) | (a[4] << 32)
    }

    pub fn format(&self, matches: MatchList) -> String {
        let mut fmt = String::new();
        for (m, c) in matches.iter().zip(self.0) {
            fmt = format!("{fmt}{}", m.format_char(c));
        }
        fmt
    }
}

impl From<[char; 5]> for Word {
    fn from(value: [char; 5]) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for Word {
    type Error = WordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars = value.chars().collect::<Vec<_>>();
        if chars.len() < 5 {
            return Err(WordError::TooSmall(chars.len()));
        }
        Ok(Self([chars[0], chars[1], chars[2], chars[3], chars[4]]))
    }
}

impl LetterMatch {
    pub fn is_correct(self) -> bool {
        self == Self::Correct
    }
    pub fn is_present(self) -> bool {
        self == Self::Present
    }
    pub fn is_absent(self) -> bool {
        self == Self::Absent
    }

    pub fn format_char(&self, c: char) -> String {
        match self {
            Self::Correct => c.green().to_string(),
            Self::Present => c.yellow().to_string(),
            Self::Absent => c.to_string(),
        }
    }
}

// Letter match extension trait

trait LetterMatchList {
    fn correct_all() -> Self;
    fn present_all() -> Self;
    fn absent_all() -> Self;
}

impl LetterMatchList for [LetterMatch; 5] {
    fn correct_all() -> Self {
        use LetterMatch as LM;
        [
            LM::Correct,
            LM::Correct,
            LM::Correct,
            LM::Correct,
            LM::Correct,
        ]
    }

    fn present_all() -> Self {
        use LetterMatch as LM;
        [
            LM::Present,
            LM::Present,
            LM::Present,
            LM::Present,
            LM::Present,
        ]
    }

    fn absent_all() -> Self {
        use LetterMatch as LM;
        [LM::Absent, LM::Absent, LM::Absent, LM::Absent, LM::Absent]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type MatchList = [LetterMatch; 5];

    use color_eyre::eyre::Result;
    use LetterMatch as LM;

    #[test]
    fn try_match() -> Result<()> {
        let fuzzy = Word::try_from("fuzzy")?;
        // All absent
        assert_eq!(
            fuzzy.match_word(&Word::try_from("hello")?),
            MatchList::absent_all(),
        );
        // One correct
        assert_eq!(
            fuzzy.match_word(&Word::try_from("testy")?),
            [LM::Absent, LM::Absent, LM::Absent, LM::Absent, LM::Correct,]
        );
        // Present
        assert_eq!(
            fuzzy.match_word(&Word::try_from("u00u0")?),
            [LM::Present, LM::Absent, LM::Absent, LM::Absent, LM::Absent],
        );
        Ok(())
    }

    #[test]
    fn test_hash() -> Result<()> {
        assert_eq!(Word::try_from("AAAAA")?.hash(), 0x0000004141414141);
        assert_eq!(Word::try_from("aaaaa")?.hash(), 0x0000006161616161);
        Ok(())
    }

    #[test]
    fn print() -> Result<()> {
        let fuzzy = Word::try_from("fuzzy")?;
        let words = [
            &Word::try_from("testy")?,
            &Word::try_from("hello")?,
            &Word::try_from("fizyk")?,
        ];
        for w in &words {
            let matches = fuzzy.match_word(w);
            println!("{}", w.format(matches));
        }
        Ok(())
    }
}
