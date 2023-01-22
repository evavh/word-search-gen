use color_eyre::eyre::Result;

mod field;
mod difficulty;

fn main() -> Result<()>{
    color_eyre::install()?;

    let difficulty = difficulty::define_difficulty();
    let field = field::Field::new(5, 4)?;
    println!("{}", field);
    println!("{:?}", difficulty);

    Ok(())
}
