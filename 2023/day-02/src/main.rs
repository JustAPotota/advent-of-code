use std::env;

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

fn part1(input: &str) {
    let (max_red, max_green, max_blue) = (12, 13, 14);

    let mut total_possible = 0u16;
    for line in input.lines() {
        let line = line.strip_prefix("Game ").unwrap();

        let game_number_length = line.find(':').unwrap();
        let game_number: u16 = line[..game_number_length].parse().unwrap();
        let line = &line[game_number_length + 1..]; // skip number and colon

        let mut is_possible = true;
        for round in line.split(';') {
            println!("{game_number}: {round}");
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for record in round.split(',') {
                let record = &record[1..]; // skip leading space
                let parts: Vec<&str> = record.split(' ').collect();
                let cube_amount: u16 = parts[0].parse().unwrap();
                let cube_color = parts[1];
                match cube_color {
                    "red" => red = cube_amount,
                    "green" => green = cube_amount,
                    "blue" => blue = cube_amount,
                    _ => panic!("Invalid color name {cube_color}"),
                }
            }

            if red > max_red || green > max_green || blue > max_blue {
                is_possible = false;
                break;
            }
        }
        if is_possible {
            total_possible += game_number;
        }
    }
    println!("{total_possible}");
    return;
}

fn part2(input: &str) {
    let mut total_power = 0;
    for line in input.lines() {
        let line = line.strip_prefix("Game ").unwrap();

        let game_number_length = line.find(':').unwrap();
        let game_number: u16 = line[..game_number_length].parse().unwrap();
        let line = &line[game_number_length + 1..]; // skip number and colon

        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        for round in line.split(';') {
            println!("{game_number}: {round}");
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for record in round.split(',') {
                let record = &record[1..]; // skip leading space
                let parts: Vec<&str> = record.split(' ').collect();
                let cube_amount: u16 = parts[0].parse().unwrap();
                let cube_color = parts[1];
                match cube_color {
                    "red" => red = cube_amount,
                    "green" => green = cube_amount,
                    "blue" => blue = cube_amount,
                    _ => panic!("Invalid color name {cube_color}"),
                }
            }
            max_red = max_red.max(red);
            max_green = max_green.max(green);
            max_blue = max_blue.max(blue);
        }
        total_power += max_red * max_green * max_blue;
    }
    println!("{total_power}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part: u8 = args.get(1).unwrap().parse().unwrap();
    let input = include_str!("input.txt");
    if part == 1 {
        part1(input);
    } else if part == 2 {
        part2(input);
    }
}
