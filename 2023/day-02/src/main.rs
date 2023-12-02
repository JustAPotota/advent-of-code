use std::{env, fs};

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

enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

struct Game {
    number: u8,
    rounds: Vec<Round>,
}

mod parser {
    use nom::branch::{alt, permutation};
    use nom::character::complete::{char, space0, space1};
    use nom::combinator::opt;
    use nom::error::ParseError;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::Tuple;
    use nom::{
        bytes::complete::tag,
        character::complete::{u16, u8},
        IResult, Parser,
    };

    use crate::{Color, Game, Round};

    fn red(input: &str) -> IResult<&str, (u8, Color)> {
        let (input, (_, count, _, _, _)) =
            (space0, u8, space0, tag("red"), opt(char(','))).parse(input)?;
        Ok((input, (count, Color::Red)))
    }

    fn green(input: &str) -> IResult<&str, (u8, Color)> {
        let (input, (_, count, _, _, _)) =
            (space0, u8, space0, tag("green"), opt(char(','))).parse(input)?;
        Ok((input, (count, Color::Green)))
    }

    fn blue(input: &str) -> IResult<&str, (u8, Color)> {
        let (input, (_, count, _, _, _)) =
            (space0, u8, space0, tag("blue"), opt(char(','))).parse(input)?;
        Ok((input, (count, Color::Blue)))
    }

    fn round(input: &str) -> IResult<&str, Round> {
        let (input, (_, colors)) =
            (space0, separated_list1(char(','), alt((red, green, blue)))).parse(input)?;
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for (count, color) in colors {
            match color {
                Color::Red => red = count,
                Color::Green => green = count,
                Color::Blue => blue = count,
            }
        }
        Ok((
            input,
            Round {
                red,   //.unwrap_or_default(),
                green, //.unwrap_or_default(),
                blue,  //.unwrap_or_default(),
            },
        ))
    }

    pub fn parse_line(line: &str) -> IResult<&str, Game> {
        let (input, (_, number, _, rounds)) = (
            tag("Game "),
            u8,
            char(':'),
            separated_list1(char(';'), round),
        )
            .parse(line)?;
        Ok((input, Game { number, rounds }))
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
