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

    pub(crate) fn try_add(&mut self,
        word: String,
        position: (usize, usize),
        directions: &Vec<Direction>,
    ) -> Result<(), WordAddError> {

        Ok(())
    }
}
