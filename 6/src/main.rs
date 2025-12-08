use std::{char, fs::read_to_string};

fn solve_first_part(file: &str) -> i64 {
    let mut iter = file.lines().rev().map(|l| l.split_whitespace());
    let operations: Vec<&str> = iter.next().unwrap().collect();

    let grid: Vec<Vec<i64>> = iter
        .rev()
        .map(|line| line.map(|n| n.parse().unwrap()).collect())
        .collect();

    (0..grid[0].len())
        .map(|x| {
            let nums = (0..grid.len()).map(|y| grid[y][x]);
            match operations[x] {
                "+" => nums.sum::<i64>(),
                "*" => nums.product::<i64>(),
                _ => panic!("operation must be + or *"),
            }
        })
        .sum()
}

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let cols = grid[0].len();

    (0..cols)
        .map(|c| grid.iter().map(|row| row[c]).collect())
        .collect()
}

fn solve_second_part(file: &str) -> i64 {
    let mut lines: Vec<&str> = file.lines().collect();
    let operations: Vec<&str> = lines.pop().unwrap().split_whitespace().collect();

    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let grid = transpose(grid);

    grid.into_iter()
        .map(|chars| chars.into_iter().filter(|c| *c != ' ').collect::<String>())
        .collect::<Vec<String>>()
        .split(|str| str.is_empty())
        .zip(operations)
        .map(|(numbers, op)| {
            let nums = numbers.iter().map(|num| num.parse::<i64>().unwrap());

            match op {
                "+" => nums.sum::<i64>(),
                "*" => nums.product::<i64>(),
                _ => panic!("operation must be + or *"),
            }
        })
        .sum()
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

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

    #[test]
    fn test_first_part() {
        assert_eq!(solve_first_part(&INPUT), 4277556);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(solve_second_part(&INPUT), 3263827);
    }
}
