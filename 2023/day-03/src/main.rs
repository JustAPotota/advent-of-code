#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test_input.txt");
        part1(input);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test_input.txt");
        part2(input);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PartNumber {
    number: u32,
    y_pos: usize,
    start_index: usize,
    end_index: usize,
}

fn find_numbers(input: &str) -> Vec<PartNumber> {
    let line_length = input.lines().next().unwrap().len();

    let mut numbers = Vec::new();
    let mut current_digits = String::with_capacity(3);
    for (index, char) in input.char_indices() {
        if char.is_numeric() {
            current_digits.push(char);
        } else if current_digits.len() > 0 {
            let number = current_digits.parse().unwrap();
            let length = current_digits.len();
            let start_index = index - length + 1;
            numbers.push(PartNumber {
                number,
                y_pos: start_index / line_length,
                start_index: start_index - 1, // dont ask why these subtract 1
                end_index: start_index + length - 1,
            });

            current_digits.clear();
        }
    }

    numbers
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit() && !c.is_whitespace()
}

fn part1(input: &str) {
    let line_length = input.lines().next().unwrap().len() + 1;
    let line_count = input.lines().count();
    let part_numbers = find_numbers(input);

    println!("{part_numbers:?}");

    let mut total = 0u32;
    for part_number in part_numbers {
        let top_edge = part_number.y_pos == 0;
        let bottom_edge = part_number.y_pos == line_count;
        let left_edge = part_number.start_index % line_length == 0;
        let right_edge = part_number.start_index % line_length == line_length;

        if !top_edge {
            if input[part_number.start_index - line_length..part_number.end_index - line_length]
                .find(is_symbol)
                .is_some()
            {
                println!("Part #{} counts u", part_number.number);
                total += part_number.number;
                continue;
            }
        }

        if !bottom_edge {
            if input[part_number.start_index + line_length..part_number.end_index + line_length]
                .find(is_symbol)
                .is_some()
            {
                println!("Part #{} counts d", part_number.number);
                total += part_number.number;
                continue;
            }
        }

        if !left_edge {
            if is_symbol(input.chars().nth(part_number.start_index - 1).unwrap()) {
                println!("Part #{} counts l", part_number.number);
                total += part_number.number;
                continue;
            }
        }

        if !right_edge {
            if is_symbol(input.chars().nth(part_number.end_index).unwrap()) {
                println!("Part #{} counts r", part_number.number);
                total += part_number.number;
                continue;
            }
        }

        if !left_edge && !top_edge {
            if is_symbol(
                input
                    .chars()
                    .nth(part_number.start_index - line_length - 1)
                    .unwrap(),
            ) {
                println!("Part #{} counts ul", part_number.number);
                total += part_number.number;
                continue;
            }
        }

        if !right_edge && !top_edge {
            if is_symbol(
                input
                    .chars()
                    .nth(part_number.end_index - line_length)
                    .unwrap(),
            ) {
                println!("Part #{} counts ur", part_number.number);
                total += part_number.number;
                continue;
            }
        }

        if !left_edge && !bottom_edge {
            if is_symbol(
                input
                    .chars()
                    .nth(part_number.start_index + line_length - 1)
                    .unwrap(),
            ) {
                println!("Part #{} counts dl", part_number.number);
                total += part_number.number;
                continue;
            }
        }

        if !right_edge && !bottom_edge {
            if is_symbol(
                input
                    .chars()
                    .nth(part_number.end_index + line_length)
                    .unwrap(),
            ) {
                println!("Part #{} counts dr", part_number.number);
                total += part_number.number;
                continue;
            }
        }
    }
    println!("{total}");
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_digit_at(input: &str, index: usize) -> bool {
    input.chars().nth(index).is_some_and(is_digit)
}

fn find_part_at<'a>(
    input: &str,
    part_numbers: &'a [PartNumber],
    found_parts: &[&PartNumber],
    index: usize,
) -> Option<&'a PartNumber> {
    if is_digit_at(input, index) {
        part_numbers.iter().find(|part_number| {
            (part_number.start_index..part_number.end_index).contains(&index)
                && !found_parts.contains(&part_number)
        })
    } else {
        None
    }
}

fn part2(input: &str) {
    let line_length = input.lines().next().unwrap().len() + 1;
    let part_numbers = find_numbers(input);

    let mut total = 0;
    for (index, _) in input.match_indices('*') {
        let mut adjacent_parts = Vec::new();

        if let Some(part) = find_part_at(
            input,
            &part_numbers,
            &adjacent_parts,
            index - line_length - 1,
        ) {
            // up
            adjacent_parts.push(part);
        }
        if let Some(part) = find_part_at(
            input,
            &part_numbers,
            &adjacent_parts,
            index + line_length - 1,
        ) {
            // down
            adjacent_parts.push(part);
        }
        if let Some(part) = find_part_at(input, &part_numbers, &adjacent_parts, index - 1) {
            // left
            adjacent_parts.push(part);
        }
        if let Some(part) = find_part_at(input, &part_numbers, &adjacent_parts, index + 1) {
            // right
            adjacent_parts.push(part);
        }
        if let Some(part) = find_part_at(
            input,
            &part_numbers,
            &adjacent_parts,
            index - line_length - 1,
        ) {
            // up left
            adjacent_parts.push(part);
        }
        if let Some(part) = find_part_at(
            input,
            &part_numbers,
            &adjacent_parts,
            index - line_length + 1,
        ) {
            // up right
            adjacent_parts.push(part);
        }
        if let Some(part) = find_part_at(
            input,
            &part_numbers,
            &adjacent_parts,
            index + line_length - 1,
        ) {
            // down left
            adjacent_parts.push(part);
        }
        if let Some(part) = find_part_at(
            input,
            &part_numbers,
            &adjacent_parts,
            index + line_length + 1,
        ) {
            // down right
            adjacent_parts.push(part);
        }

        println!("{adjacent_parts:?}");

        if adjacent_parts.len() == 2 {
            total += adjacent_parts[0].number * adjacent_parts[1].number;
        }
    }
    println!("{total}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let part: u8 = args.get(1).unwrap().parse().unwrap();
    let input = include_str!("input.txt");
    if part == 1 {
        part1(input);
    } else if part == 2 {
        part2(input);
    }
}
