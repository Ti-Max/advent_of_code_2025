use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("data/input")?;
    let _lines: Vec<&str> = file.lines().collect();

    Ok(())
}
