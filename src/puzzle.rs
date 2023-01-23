use std::fmt;

use thiserror::Error;

use crate::difficulty::{Difficulty, Direction};
use crate::field::{Field, WordAddError};

#[derive(Error, Debug)]
pub(crate) enum PuzzleError {}

pub(crate) struct Puzzle<'a> {
    field: Field<'a>,
    difficulty: Difficulty,
    directions: Vec<Direction>,
}

impl fmt::Display for Puzzle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Difficulty: {}\n{}", self.difficulty, self.field)
    }
}

impl Puzzle<'_> {
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

    pub(crate) fn fill(
        &mut self,
        word_list: Vec<&str>,
    ) -> Result<(), PuzzleError> {
        for word in &word_list {
            let added_word =
                self.field.try_add(word, &self.directions);
            match added_word {
                Err(WordAddError::DoesntFit) => todo!("Implement backtrack"),
                Ok(()) => (),
            }
        }
        Ok(())
    }
}
