use std::fmt;

use thiserror::Error;

use crate::difficulty::{Difficulty, Direction};
use crate::field::{Field, WordAddError};

#[derive(Error, Debug)]
pub(crate) enum PuzzleError {}

pub(crate) struct Puzzle {
    field: Field,
    difficulty: Difficulty,
    directions: Vec<Direction>,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Difficulty: {}\n{}", self.difficulty, self.field)
    }
}

impl Puzzle {
    pub(crate) fn new(
        lines: usize,
        columns: usize,
        difficulty: Difficulty,
    ) -> Self {
        let field = Field::new(lines, columns);
        let directions = difficulty.directions();

        Self {
            field,
            difficulty,
            directions,
        }
    }

    //Pre-condition: word_list contains no words with word.len() > isize::MAX
    pub(crate) fn fill<R: rand::Rng>(
        &mut self,
        mut rng: R,
        word_list: Vec<&str>,
    ) -> Result<(), PuzzleError> {
        for word in &word_list {
            let added_word =
                self.field.try_add(&mut rng, word, &self.directions);
            match added_word {
                Err(WordAddError::DoesntFit) => todo!("Implement backtrack"),
                Ok(()) => (),
            }
        }
        Ok(())
    }
}
