mod lsystem;
mod note;
mod parser;

fn main() -> Result<(), &'static str> {
    let mut system = crate::parser::parse_file("example.lsys")?;

    system.forward(4);

    println!("{system}");

    Ok(())
}
