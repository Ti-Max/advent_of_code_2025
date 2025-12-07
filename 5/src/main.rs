use std::{char, fs, usize};

// should be facter than Vec<Vec<>>
struct Grid<T: PartialEq> {
    width: usize,
    #[allow(unused)]
    height: usize,
    arr: Vec<T>,
}

impl<T: PartialEq> Grid<T> {
    fn from_file(file: &str) -> Grid<char> {
        let lines: Vec<&str> = file.lines().collect();
        let height = lines.len();
        let width = lines
            .iter()
            .next()
            .expect("file should not be empty")
            .trim()
            .len();

        let mut arr = vec![];
        for line in lines {
            arr.extend(line.trim().chars());
        }

        Grid::<char> { width, height, arr }
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.arr[x + y * self.width]
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        self.arr[x + y * self.width] = value;
    }

    fn count_adjacent_items(&self, x: usize, y: usize, compare_item: T) -> usize {
        // @ @ @
        // @ X @
        // @ @ @
        let offsets: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        offsets
            .into_iter()
            .filter(|(offset_x, offset_y)| {
                let x = (x as isize) + offset_x;
                let y = (y as isize) + offset_y;
                if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
                    *(self.get(x as usize, y as usize)) == compare_item
                } else {
                    false
                }
            })
            .count()
    }
}

fn solve_first_part(grid: &Grid<char>) -> u32 {
    let mut rolls = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if *(grid.get(x, y)) == '@' {
                if grid.count_adjacent_items(x, y, '@') < 4 {
                    rolls += 1;
                }
            }
        }
    }
    rolls
}

fn solve_second_part(mut grid: Grid<char>, rolls_count: u32) -> (Grid<char>, u32) {
    let mut rolls = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if *(grid.get(x, y)) == '@' {
                if grid.count_adjacent_items(x, y, '@') < 4 {
                    grid.set(x, y, '.');
                    rolls += 1;
                }
            }
        }
    }
    if rolls > 0 {
        solve_second_part(grid, rolls_count + rolls)
    } else {
        (grid, rolls_count + rolls)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("data/input")?;

    let grid = Grid::<char>::from_file(&file);

    let first_answer = solve_first_part(&grid);
    let (_grid, second_answer) = solve_second_part(grid, 0);

    println!("First answer: {}", first_answer);
    println!("Second answer: {}", second_answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Grid, solve_first_part, solve_second_part};

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_grid() {
        let grid = Grid::<char>::from_file(INPUT);
        assert_eq!(*grid.get(0, 0), '.');
        assert_eq!(*grid.get(0, 1), '@');
        assert_eq!(*grid.get(3, 1), '.');
    }

    #[test]
    fn test_first_part() {
        let grid = Grid::<char>::from_file(INPUT);

        assert_eq!(solve_first_part(&grid), 13);
    }

    #[test]
    fn test_second_part() {
        let grid = Grid::<char>::from_file(INPUT);
        let (_, rolls) = solve_second_part(grid, 0);
        assert_eq!(rolls, 43);
    }
}
