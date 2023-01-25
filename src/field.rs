use std::{fmt, ops::Range};

use thiserror::Error;

use crate::difficulty::Direction;

#[derive(Error, Debug, PartialEq, Eq)]
pub(crate) enum WordAddError {
    #[error("word cannot be fit into field")]
    DoesntFit,
}

#[derive(Debug, PartialEq, Eq)]
struct PosRanges {
    rows: Range<usize>,
    cols: Range<usize>,
}

impl PosRanges {
    fn new(
        rows: Range<usize>,
        cols: Range<usize>,
    ) -> Self {
        Self { rows, cols }
    }
}

pub(crate) struct Field<'a> {
    n_rows: usize,
    n_cols: usize,
    grid: Vec<Vec<&'a str>>,
}

impl fmt::Display for Field<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_grid = self
            .grid
            .clone()
            .into_iter()
            .map(|x| x.join(" "))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}\n", formatted_grid)
    }
}

impl Field<'_> {
    pub(crate) fn new(n_rows: usize, n_cols: usize) -> Self {
        let grid = vec![vec!["_"; n_cols]; n_rows];

        Self { n_rows, n_cols, grid }
    }

    fn get_possible_positions(
        &self,
        word: &str,
        direction: &Direction,
    ) -> Result<PosRanges, WordAddError> {
        use Direction::*;

        let rows = self.n_rows;
        let cols = self.n_cols;

        if (word.len() > cols && direction != &Up && direction != &Down)
            || (word.len() > rows && direction != &Right && direction != &Left)
        {
            return Err(WordAddError::DoesntFit);
        }

        let all_rows = 0..rows;
        let all_cols = 0..cols;
        let right_cols = 0..(cols - word.len() + 1);
        let left_cols = (word.len() - 1)..cols;
        let down_rows = 0..(rows - word.len() + 1);
        let up_rows = (word.len() - 1)..rows;

        Ok(match direction {
            Right => PosRanges::new(all_rows, right_cols),
            Left => PosRanges::new(all_rows, left_cols),
            Down => PosRanges::new(down_rows, all_cols),
            Up => PosRanges::new(up_rows, all_cols),
            RightUp => PosRanges::new(up_rows, right_cols),
            RightDown => PosRanges::new(down_rows, right_cols),
            LeftUp => PosRanges::new(up_rows, left_cols),
            LeftDown => PosRanges::new(down_rows, left_cols),
        })
    }

    pub(crate) fn try_add(
        &mut self,
        word: &str,
        directions: &Vec<Direction>,
    ) -> Result<(), WordAddError> {
        for direction in directions {
            let positions = self.get_possible_positions(word, direction)?;
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::difficulty::Difficulty;

    #[test]
    fn try_add_possible() {
        let mut field = Field::new(7, 8);
        let word = "egg";

        for direction in Difficulty::Hard.directions() {
            assert!(field.try_add(word, &vec![direction]).is_ok());
        }
    }

    #[test]
    fn try_add_impossible() {
        let mut field = Field::new(7, 8);
        let word = "notpossss";

        for direction in Difficulty::Hard.directions() {
            assert!(field.try_add(word, &vec![direction]).is_err());
        }
    }

    #[test]
    fn possible_positions() {
        use Direction::*;

        let field = Field::new(7, 8);
        let word = "egg";

        for direction in Difficulty::Hard.directions() {
            let positions =
                field.get_possible_positions(word, &direction).unwrap();
            let correct_positions = match direction {
                // + 1 to make it an inclusive range without changing type
                Right => PosRanges::new(0..6 + 1, 0..5 + 1),
                Left => PosRanges::new(0..6 + 1, 2..7 + 1),
                Up => PosRanges::new(2..6 + 1, 0..7 + 1),
                Down => PosRanges::new(0..4 + 1, 0..7 + 1),
                RightUp => PosRanges::new(2..6 + 1, 0..5 + 1),
                RightDown => PosRanges::new(0..4 + 1, 0..5 + 1),
                LeftUp => PosRanges::new(2..6 + 1, 2..7 + 1),
                LeftDown => PosRanges::new(0..4 + 1, 2..7 + 1),
            };

            dbg!(direction);
            assert_eq!(positions, correct_positions);
        }
    }
}
