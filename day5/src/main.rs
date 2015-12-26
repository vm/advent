use std::io::prelude::*;
use std::fs::File;

struct Word {
    word: String,
    current_char: Option<char>,
    previous_char: Option<char>,
    consecutive_same_letter: bool,
    num_vowels: usize,
}

impl Word {
    fn new(word: String) -> Word {
        Word {
            word: word,
            current_char: None,
            previous_char: None,
            consecutive_same_letter: false,
            num_vowels: 0,
        }
    }

    fn is_nice(&mut self) -> bool {
        for c in self.word.clone().chars() {
            self.previous_char = self.current_char;
            self.current_char = Some(c);
            self.check_consecutive_same();
            self.check_is_vowel();
            if self.is_illegal_pair() {
                return false
            }
        }
        self.consecutive_same_letter && self.num_vowels == 3
    }

    fn check_consecutive_same(&mut self) {
        if !self.consecutive_same_letter && self.previous_char == self.current_char {
            self.consecutive_same_letter = true;
        }
    }

    fn is_illegal_pair(&self) -> bool {
        let illegals = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
        match (self.previous_char, self.current_char) {
            (Some(ref previous), Some(ref current)) => illegals.contains(&(*previous, *current)),
            _ => false,
        }
    }

    fn check_is_vowel(&mut self) {
        if let Some(ref current) = self.current_char {
            if self.num_vowels < 3 && ['a', 'e', 'i', 'o', 'u'].contains(&*current) {
                self.num_vowels += 1
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("./input.txt").unwrap();
    file.read_to_string(&mut input);
    let num_nice_strings = input
        .lines()
        .filter(|line| Word::new(line.to_owned().to_owned()).is_nice())
        .count();
    println!("num of nice strings: {}", num_nice_strings)
}
