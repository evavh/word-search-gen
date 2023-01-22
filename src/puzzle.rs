use std::collections::HashSet;
use std::fmt;

use crate::difficulty::{define_difficulty, Difficulty, Direction};
use crate::field::Field;

pub(crate) struct Puzzle<'a> {
    field: Field<'a>,
    difficulty: Difficulty,
    directions: HashSet<Direction>,
}

impl fmt::Display for Puzzle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Difficulty: {}\n{}\n", self.difficulty, self.field)
    }
}

impl Puzzle<'_> {
    pub(crate) fn new(
        lines: usize,
        columns: usize,
        difficulty: Difficulty,
    ) -> Self {
        let field = Field::new(lines, columns);
        let directions = define_difficulty()
            .clone()
            .get(&difficulty)
            .expect("Every possible difficulty should be defined.")
            .clone();

        Self {
            field,
            difficulty,
            directions,
        }
    }
}
