use color_eyre::eyre::Result;

mod field;

fn main() -> Result<()>{
    color_eyre::install()?;

    let field = field::Field::new(5, 4)?;
    print!("{}\n", field);

    Ok(())
}
