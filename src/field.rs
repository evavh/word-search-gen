use std::{
    fmt,
    ops::{Add, Range},
};

use itertools::Itertools;
use thiserror::Error;

use crate::difficulty::Direction;

const DEFAULT_CHAR: char = '_';

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
    fn new(rows: Range<usize>, cols: Range<usize>) -> Self {
        Self { rows, cols }
    }
}

#[derive(Clone, Debug, Copy)]
pub(crate) struct Coordinate {
    row: isize,
    col: isize,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        let row = self.row + rhs.row;
        let col = self.col + rhs.col;
        Coordinate { row, col }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n", self.row, self.col)
    }
}

impl Coordinate {
    fn new(row: isize, col: isize) -> Self {
        Coordinate { row, col }
    }
}

struct CoordinateGenerator {
    n_rows: isize,
    n_cols: isize,
    curr: Coordinate,
    offset: Coordinate,
}

impl Iterator for CoordinateGenerator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.row >= self.n_rows
            || self.curr.col >= self.n_cols
            || self.curr.row < 0
            || self.curr.col < 0
        {
            None
        } else {
            let old = self.curr;
            let new = old.clone() + self.offset.clone();
            self.curr = new.clone();
            Some(old)
        }
    }
}

impl CoordinateGenerator {
    fn new(
        field: &Field,
        start_coordinate: Coordinate,
        direction: &Direction,
    ) -> Self {
        let offset = match direction {
            Direction::Right => Coordinate::new(0, 1),
            Direction::Left => Coordinate::new(0, -1),
            Direction::Up => Coordinate::new(-1, 0),
            Direction::Down => Coordinate::new(1, 0),
            Direction::RightUp => Coordinate::new(-1, 1),
            Direction::RightDown => Coordinate::new(1, 1),
            Direction::LeftUp => Coordinate::new(-1, -1),
            Direction::LeftDown => Coordinate::new(1, -1),
        };

        CoordinateGenerator {
            n_rows: field.n_rows.try_into().unwrap(),
            n_cols: field.n_cols.try_into().unwrap(),
            curr: start_coordinate,
            offset,
        }
    }
}

pub(crate) struct Field {
    n_rows: usize,
    n_cols: usize,
    grid: Vec<Vec<char>>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_grid = self
            .grid
            .clone()
            .into_iter()
            .map(|x| x.iter().map(char::to_string).join(" "))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}\n", formatted_grid)
    }
}

impl Field {
    pub(crate) fn new(n_rows: usize, n_cols: usize) -> Self {
        let _i_rows: isize = n_rows.try_into().unwrap();
        let _i_cols: isize = n_cols.try_into().unwrap();

        let grid = vec![vec![DEFAULT_CHAR; n_cols]; n_rows];

        Self {
            n_rows,
            n_cols,
            grid,
        }
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

    // Cannot crash because all usizes used should fit into isizes
    pub(crate) fn try_add<R: rand::Rng>(
        &mut self,
        mut rng: R,
        word: &str,
        directions: &Vec<Direction>,
    ) -> Result<(), WordAddError> {
        use rand::seq::SliceRandom;

        for direction in directions {
            let positions = self.get_possible_positions(word, direction)?;

            let mut start_coordinates = positions
                .rows
                .cartesian_product(positions.cols)
                .map(|(r, c)| {
                    Coordinate::new(
                        r.try_into().unwrap(),
                        c.try_into().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            start_coordinates.shuffle(&mut rng);

            for start_coordinate in start_coordinates {
                match self.try_fit(word, direction, &start_coordinate) {
                    Err(WordAddError::DoesntFit) => continue,
                    Ok(()) => {
                        self.put_into_grid(word, direction, &start_coordinate);
                        return Ok(());
                    }
                };
            }
        }
        Ok(())
    }

    fn try_fit(
        &self,
        word: &str,
        direction: &Direction,
        start_coordinate: &Coordinate,
    ) -> Result<(), WordAddError> {
        let coordinates = CoordinateGenerator::new(
            &self,
            start_coordinate.clone(),
            direction,
        );

        for (i, coordinate) in coordinates.enumerate() {
            println!("Trying to fit letter {i} of word {word} at {coordinate}");
            if i < word.len() {
                let row: usize = coordinate.row.try_into().unwrap();
                let col: usize = coordinate.col.try_into().unwrap();

                let word_letter = word.chars().nth(i).unwrap();
                let field_letter = self.grid[row][col];

                if field_letter == DEFAULT_CHAR {
                    continue;
                } else if word_letter != field_letter {
                    return Err(WordAddError::DoesntFit);
                }
            } else {
                return Ok(());
            }
        }

        Err(WordAddError::DoesntFit)
    }

    fn put_into_grid(
        &mut self,
        word: &str,
        direction: &Direction,
        start_coordinate: &Coordinate,
    ) {
        let coordinates = CoordinateGenerator::new(
            &self,
            start_coordinate.clone(),
            direction,
        );

        for (i, coordinate) in coordinates.enumerate() {
            let row: usize = dbg!(coordinate.row).try_into().unwrap();
            let col: usize = dbg!(coordinate.col).try_into().unwrap();

            let Some(letter) = word.chars().nth(i) else {
                return;
            };
            println!("Putting letter {letter} in grid");
            self.grid[row][col] = letter;
        }
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
        let mut rng = rand::thread_rng();

        for direction in Difficulty::Hard.directions() {
            assert!(field.try_add(&mut rng, word, &vec![direction]).is_ok());
        }
    }

    #[test]
    fn try_add_impossible() {
        let mut field = Field::new(7, 8);
        let word = "notpossss";
        let mut rng = rand::thread_rng();

        for direction in Difficulty::Hard.directions() {
            assert!(field.try_add(&mut rng, word, &vec![direction]).is_err());
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

            assert_eq!(positions, correct_positions);
        }
    }
}
