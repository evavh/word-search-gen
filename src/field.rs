use std::fmt;
use color_eyre::eyre::Result;

pub struct Field<'a> {
    lines: usize,
    columns : usize,
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

        write!(f, "{}", formatted_grid)
    }
}

impl Field<'_> {
    pub fn new(lines: usize, columns: usize) -> Result<Self>  {
        let mut grid = Vec::with_capacity(lines);
        for _ in 1..lines {
            grid.push(Vec::with_capacity(columns));
        }

        Ok(Self { lines, columns, grid })
    }
}

