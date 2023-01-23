use std::{fmt, ops::Range};

use thiserror::Error;

use crate::difficulty::Direction;

#[derive(Error, Debug, PartialEq, Eq)]
pub(crate) enum WordAddError {
    #[error("word cannot be fit into field")]
    DoesntFit,
}

pub(crate) struct Field<'a> {
    lines: usize,
    columns: usize,
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
    pub(crate) fn new(lines: usize, columns: usize) -> Self {
        let grid = vec![vec!["_"; columns]; lines];

        Self {
            lines,
            columns,
            grid,
        }
    }

    fn get_possible_positions(
        &self,
        word: &str,
        direction: &Direction,
    ) -> Result<(Range<usize>, Range<usize>), WordAddError> {
        use Direction::*;

        let lines = self.lines;
        let columns = self.columns;

        if (word.len() > columns && direction != &Up && direction != &Down)
            || (word.len() > lines && direction != &Right && direction != &Left)
        {
            return Err(WordAddError::DoesntFit);
        }

        let all_lines = 0..lines;
        let all_cols = 0..columns;
        let right_col_range = 0..(columns - word.len() + 1);
        let left_col_range = (word.len() - 1)..columns;
        let down_line_range = 0..(lines - word.len() + 1);
        let up_line_range = (word.len() - 1)..lines;

        Ok(match direction {
            Right => (all_lines, right_col_range),
            Left => (all_lines, left_col_range),
            Down => (down_line_range, all_cols),
            Up => (up_line_range, all_cols),
            RightUp => (up_line_range, right_col_range),
            RightDown => (down_line_range, right_col_range),
            LeftUp => (up_line_range, left_col_range),
            LeftDown => (down_line_range, left_col_range),
        })
    }

    pub(crate) fn try_add(
        &mut self,
        word: &str,
        directions: &Vec<Direction>,
    ) -> Result<(), WordAddError> {
        for direction in directions {
            let pos_ranges = self.get_possible_positions(word, direction)?;
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
            let pos_ranges =
                field.get_possible_positions(word, &direction).unwrap();
            let correct_pos_ranges = match direction {
                // + 1 to make it an inclusive range without changing type
                Right => (0..6 + 1, 0..5 + 1),
                Left => (0..6 + 1, 2..7 + 1),
                Up => (2..6 + 1, 0..7 + 1),
                Down => (0..4 + 1, 0..7 + 1),
                RightUp => (2..6 + 1, 0..5 + 1),
                RightDown => (0..4 + 1, 0..5 + 1),
                LeftUp => (2..6 + 1, 2..7 + 1),
                LeftDown => (0..4 + 1, 2..7 + 1),
            };

            dbg!(direction);
            assert_eq!(pos_ranges, correct_pos_ranges);
        }
    }
}
