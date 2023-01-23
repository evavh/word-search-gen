use color_eyre::eyre::Result;

use crate::difficulty::Difficulty;
use crate::puzzle::Puzzle;

mod difficulty;
mod field;
mod puzzle;

fn main() -> Result<()> {
    color_eyre::install()?;

    let puzzle = Puzzle::new(5, 4, Difficulty::Normal);
    println!("{}", puzzle);

    Ok(())
}
