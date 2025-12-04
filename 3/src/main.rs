use std::{fs, usize};

fn find_largest_number_v1(line: &str) -> u64 {
    // reverse to get the first maximum digit
    let (index_from_end, first_digit) = line
        .chars()
        .rev()
        .enumerate()
        .skip(1)
        .max_by_key(|(_i, d)| *d)
        .expect("line should not be empty");

    let second_digit = line
        .chars()
        .skip(line.len() - index_from_end)
        .max()
        .expect("line shold contain at least two digits");

    [first_digit, second_digit]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

const BATTERIES_COUNT: usize = 12;

fn find_largest_number_v2(line: &str) -> u64 {
    // reverse to get the first maximum digit

    (0..BATTERIES_COUNT)
        .scan(0, |state, i| {
            let (index_from_end, digit) = line
                .chars()
                .skip(*state)
                // can't do rev() because "the trait bound is not satisfied bla bla bla"
                .collect::<String>()
                .chars()
                .rev()
                .enumerate()
                .skip(BATTERIES_COUNT - i - 1)
                .max_by_key(|(_i, d)| *d)
                .unwrap();

            *state = line.len() - index_from_end;

            Some(digit)
        })
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("data/input")?;

    let first_answer: u64 = file.lines().map(|line| find_largest_number_v1(line)).sum();
    let second_answer: u64 = file.lines().map(|line| find_largest_number_v2(line)).sum();

    println!("First answer: {}", first_answer);
    println!("Second answer: {}", second_answer);

    Ok(())
}
