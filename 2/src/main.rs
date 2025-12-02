use std::{fs, ops::RangeInclusive};

fn is_invalid_v1(id: i64) -> bool {
    let str = id.to_string();
    if str.len() % 2 == 0 {
        let middle = str.len() / 2;
        str[..middle] == str[middle..]
    } else {
        false
    }
}

fn is_invalid_v2(id: i64) -> bool {
    let id = id.to_string();
    // split the string into all possible chunks
    (2..=id.len()).any(|i| {
        if id.len() % i == 0 {
            id.as_bytes()
                .chunks(id.len() / i)
                .map(|c| -> &str { std::str::from_utf8(c).unwrap() })
                .collect::<Vec<&str>>()
                .as_slice()
                .windows(2)
                .all(|w| w[0] == w[1])
        } else {
            false
        }
    })
}

fn all_ids(s: &String) -> impl Iterator<Item = i64> {
    s.trim()
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(start, end)| -> RangeInclusive<i64> {
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .flat_map(|range| -> Vec<i64> { range.collect() })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let string = fs::read_to_string("data/input")?;

    let first_answer: i64 =
        all_ids(&string).fold(0, |acc, id| if is_invalid_v1(id) { acc + id } else { acc });

    let second_answer: i64 =
        all_ids(&string).fold(0, |acc, id| if is_invalid_v2(id) { acc + id } else { acc });

    println!("First answer: {}", first_answer);
    println!("Second answer: {}", second_answer);

    Ok(())
}
