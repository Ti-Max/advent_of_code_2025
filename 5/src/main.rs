use std::{fs::read_to_string, ops::RangeInclusive};

fn solve_first_part(file: &str) -> u32 {
    let mut lines = file.lines();

    let ranges: Vec<RangeInclusive<i64>> = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let (start, end) = line.trim().split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    lines
        .filter(|line| {
            ranges.iter().any(|range| {
                let a = &line.trim().parse::<i64>().unwrap();
                range.contains(a)
            })
        })
        .count() as u32
}

fn solve_second_part(file: &str) -> i64 {
    let lines = file.lines();

    let mut ranges: Vec<RangeInclusive<i64>> = lines
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let (start, end) = line.trim().split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    ranges.sort_by_key(|r| *r.start());

    ranges
        .into_iter()
        .scan(0, |max_end, mut r| {
            if *max_end >= *r.start() {
                r = (*max_end + 1)..=*r.end();
            }

            *max_end = (*max_end).max(*r.end());
            Some(r)
        })
        .map(|r| 0.max(*r.end() - *r.start() + 1))
        .sum::<i64>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = read_to_string("data/input")?;

    let first_answer = solve_first_part(&file);
    let second_answer = solve_second_part(&file);

    println!("First answer: {}", first_answer);
    println!("Second answer: {}", second_answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{solve_first_part, solve_second_part};

    const INPUT: &str = "3-5
5-5
12-18
17-40

1
5
8
11
17
32";

    #[test]
    fn test_first_part() {
        assert_eq!(solve_first_part(&INPUT), 3);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(solve_second_part(&INPUT), 14);
    }
}
