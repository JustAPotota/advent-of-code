use std::str::FromStr;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test_input.txt");

        let (output, time_taken) = common::time_fn(part1, input);
        println!("Got '{output}' in {time_taken:#?}");
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test_input.txt");

        let (output, time_taken) = common::time_fn(part2, input);
        println!("Got '{output}' in {time_taken:#?}");
    }
}

fn distance_traveled(ms_held: u32, race_duration: u32) -> u32 {
    let ms_travelling = race_duration - ms_held;
    ms_held * ms_travelling
}

#[derive(Debug)]
struct Race {
    duration: u32,
    record: u32,
}

fn parse_line<T>(line: &str) -> Vec<T>
where
    T: FromStr,
{
    line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let durations: Vec<u32> = parse_line(lines[0]);
    let records: Vec<u32> = parse_line(lines[1]);

    durations
        .into_iter()
        .zip(records.into_iter())
        .map(|(duration, record)| Race { duration, record })
        .collect()
}

fn part1(input: &str) -> u32 {
    let races = parse_input(input);
    races.into_iter().fold(1, |product, race| {
        let ways_to_beat_record = (1..race.duration)
            .filter(|ms_held| distance_traveled(*ms_held, race.duration) > race.record)
            .count() as u32;
        product * ways_to_beat_record
    })
}

fn parse_line2(line: &str) -> u64 {
    line.chars()
        .filter(|c| c.is_numeric())
        .fold(String::new(), |mut s, c| {
            s.push(c);
            s
        })
        .parse()
        .unwrap()
}

struct BigRace {
    duration: u64,
    record: u64,
}

impl BigRace {
    fn distance_traveled(&self, ms_held: u64) -> u64 {
        let ms_travelling = self.duration - ms_held;
        ms_held * ms_travelling
    }
}

fn parse_input2(input: &str) -> BigRace {
    let numbers: Vec<u64> = input.lines().map(parse_line2).collect();
    BigRace {
        duration: numbers[0],
        record: numbers[1],
    }
}

fn part2(input: &str) -> u32 {
    let race = parse_input2(input);
    let ways_to_beat_record = (1..race.duration)
        .filter(|ms_held| race.distance_traveled(*ms_held) > race.record)
        .count() as u32;

    ways_to_beat_record
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let part: u8 = args.get(1).unwrap().parse().unwrap();
    let input = include_str!("input.txt");
    if part == 1 {
        let (output, time_taken) = common::time_fn(part1, input);
        println!("Got '{output}' in {time_taken:#?}");
    } else if part == 2 {
        let (output, time_taken) = common::time_fn(part2, input);
        println!("Got '{output}' in {time_taken:#?}");
    }
}
