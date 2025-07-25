use color_eyre::eyre::Result;

use crate::difficulty::Difficulty;
use crate::puzzle::Puzzle;

mod difficulty;
mod field;
mod puzzle;

fn main() -> Result<()> {
    color_eyre::install()?;

    let rng = rand::thread_rng();
    let mut puzzle = Puzzle::new(5, 4, Difficulty::Hard);

    let word_list = vec!["egg", "good", "leg"];
    puzzle.fill(rng, word_list).unwrap();
    println!("{}", puzzle);

    Ok(())
}
