use std::fs;
mod dial_rotator;
mod dial_rotator_v2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("data/input")?;
    let lines: Vec<&str> = file.lines().collect();

    let first_answer = dial_rotator::rotate_and_count_zeros(&lines);
    let second_answer = dial_rotator::rotate_and_count_all_zeros(&lines);

    println!("First answer: {}", first_answer);
    println!("Second answer: {}", second_answer);

    println!("------");
    println!("V2");
    println!("------");
    let first_answer = dial_rotator_v2::rotate_and_count_zeros(&lines);
    let second_answer = dial_rotator_v2::rotate_and_count_all_zeros(lines);

    println!("First answer: {}", first_answer);
    println!("Second answer: {}", second_answer);

    Ok(())
}
