use color_eyre::eyre::Result;
use std::fmt;

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
    pub(crate) fn new(lines: usize, columns: usize) -> Result<Self> {
        let grid = vec![vec!["_"; columns]; lines];

        Ok(Self {
            lines,
            columns,
            grid,
        })
    }
}
