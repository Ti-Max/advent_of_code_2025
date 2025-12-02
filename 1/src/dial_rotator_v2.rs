const MIN_VALUE: i32 = 0;
const MAX_VALUE: i32 = 99;
const VALUE_COUNT: i32 = MAX_VALUE - MIN_VALUE + 1;
const INITIAL_VALUE: i32 = 50;

// line format:
// R67
// L6767

fn get_first_char(line: &str) -> char {
    line.chars().next().unwrap()
}

fn number_after_first_char(line: &str) -> i32 {
    line[1..].parse().unwrap()
}

pub fn rotate_and_count_zeros(lines: &Vec<&str>) -> i32 {
    lines
        .iter()
        .scan(INITIAL_VALUE, |state: &mut i32, line| {
            let direction: char = get_first_char(line);
            let count: i32 = number_after_first_char(line);

            match direction {
                'R' => *state += count,
                'L' => *state -= count,
                _ => panic!("the first character of the line is either R nor L"),
            }
            Some(*state)
        })
        .map(|state| state.rem_euclid(VALUE_COUNT))
        .fold(0, |acc, v| if v == 0 { acc + 1 } else { acc })
}

fn count_zero_ticks(position: &i32) -> i32 {
    position / VALUE_COUNT
}

// same as v1
pub fn rotate_and_count_all_zeros(lines: Vec<&str>) -> i32 {
    let mut position = INITIAL_VALUE;
    let mut zero_count = 0;
    for line in lines {
        let direction: char = get_first_char(line);
        let count: i32 = number_after_first_char(line);

        match direction {
            'R' => {
                position += count;
                zero_count += count_zero_ticks(&position);
            }
            'L' => {
                let mut fliped_position = VALUE_COUNT - position;
                fliped_position = fliped_position.rem_euclid(VALUE_COUNT);
                fliped_position += count;

                zero_count += count_zero_ticks(&fliped_position);

                position -= count;
            }
            _ => panic!("the first character of the line is either R nor L"),
        }

        position = position.rem_euclid(VALUE_COUNT);
    }

    zero_count
}
