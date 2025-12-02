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

fn normalize_position(position: i32) -> i32 {
    let mut position = position % VALUE_COUNT;

    if position < 0 {
        position = VALUE_COUNT + position;
    }

    position
}

pub fn rotate_and_count_zeros(lines: &Vec<&str>) -> i32 {
    let mut position = INITIAL_VALUE;
    let mut zero_count = 0;
    for line in lines {
        let direction: char = get_first_char(line);
        let count: i32 = number_after_first_char(line);

        match direction {
            'R' => position += count,
            'L' => position -= count,
            _ => panic!("the first character of the line is either R nor L"),
        }

        position = normalize_position(position);

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn count_zero_ticks(position: &i32) -> i32 {
    position / VALUE_COUNT
}

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
                fliped_position = normalize_position(fliped_position);
                fliped_position += count;

                zero_count += count_zero_ticks(&fliped_position);

                position -= count;
            }
            _ => panic!("the first character of the line is either R nor L"),
        }

        position = normalize_position(position);
    }

    zero_count
}
