use std::{
    collections::HashMap,
    ops::{Add, Range},
};

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

fn line_to_numbers(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

struct Map {
    in_start: u32,
    out_start: u32,
    size: u32,
}

impl Map {
    fn from_line(line: &str) -> Self {
        let numbers = line_to_numbers(line);
        let (out_start, in_start, size) = (numbers[0], numbers[1], numbers[2]);
        Self {
            in_start,
            out_start,
            size,
        }
    }

    fn get(&self, input: u32) -> Option<u32> {
        if input < self.in_start || input as u64 > (self.in_start as u64) + (self.size as u64) {
            None
        } else {
            let difference = input - self.in_start;
            Some(self.out_start + difference)
        }
    }
}

struct Converter {
    name: String,
    maps: Vec<Map>,
}

impl Converter {
    fn from_block(block: &str) -> Self {
        let mut lines = block.lines();

        let (name, _) = lines.next().unwrap().split_once(' ').unwrap();
        let name = name.to_string();

        let maps = lines.map(Map::from_line).collect();

        Self { name, maps }
    }

    fn get(&self, input: u32) -> u32 {
        for map in self.maps.iter() {
            if let Some(output) = map.get(input) {
                return output;
            }
        }
        input
    }
}

fn apply_conversions(converters: &[Converter], mut input: u32) -> u32 {
    for converter in converters {
        input = converter.get(input);
    }
    input
}

fn part1(input: &str) {
    let mut block_iter = input.split("\n\n");

    let (_, seed_numbers) = block_iter.next().unwrap().split_once(": ").unwrap();
    let seed_numbers = line_to_numbers(seed_numbers);
    println!("Seeds: {seed_numbers:?}");

    let converters: Vec<Converter> = block_iter.map(Converter::from_block).collect();

    let location_numbers: Vec<u32> = seed_numbers
        .iter()
        .map(|seed| apply_conversions(&converters, *seed))
        .collect();

    let lowest_location = location_numbers.iter().min().unwrap();
    println!("{lowest_location}");
}

struct Map2 {
    range: Range<u32>,
    offset: i64,
}

impl Map2 {
    fn convert(&self, input: u32) -> Option<u32> {
        if self.range.contains(&input) {
            Some((input as i64 + self.offset) as u32)
        } else {
            None
        }
    }
}

struct Map2Set {
    maps: Vec<Map2>,
}

fn add(a: u32, b: i64) -> u32 {
    (a as i64 + b) as u32
}

fn split_range(range: Range<u32>, at: u32) -> (Range<u32>, Range<u32>) {
    (range.start..at, at..range.end)
}

fn offset_range(range: Range<u32>, offset: i64) -> Range<u32> {
    add(range.start, offset)..add(range.end, offset)
}

impl Map2Set {
    fn aaaaa(&self, input: Range<u32>) -> Vec<Range<u32>> {
        let mut converted_ranges = Vec::new();
        let mut remaining_ranges = vec![Some(input)];

        for map in self.maps {
            for mut maybe_range in remaining_ranges {
                let range = maybe_range.unwrap();
                if range.end < map.range.start || range.start > map.range.end {
                    continue;
                }

                if range.start >= map.range.start {
                    if range.end <= map.range.end {
                        let converted_range = offset_range(range, map.offset);
                        converted_ranges.push(converted_range);
                        maybe_range = None;
                    } else {
                        let (to_convert, new_range) = split_range(range, map.range.end);
                        remaining_ranges.push(Some(new_range));
                        converted_ranges.push(offset_range(to_convert, map.offset));
                        maybe_range = None;
                    }
                }
            }
            remaining_ranges = remaining_ranges
                .into_iter()
                .filter(Option::is_some)
                .collect();
        }

        converted_ranges
    }
}

fn block_to_ranges(block: &str) -> Vec<Map2> {
    let lines = block.lines().skip(1);
    lines
        .map(|line| {
            let numbers = line_to_numbers(line);
            let (out_start, in_start, size) = (numbers[0], numbers[1], numbers[2]);
            let range = in_start..in_start + size;
            let offset = out_start as i64 - in_start as i64;
            Map2 { range, offset }
        })
        .collect()
}

fn part2(input: &str) {
    let mut blocks = input.split("\n\n");

    let (_, seed_numbers) = blocks.next().unwrap().split_once(": ").unwrap();
    let seed_numbers = line_to_numbers(seed_numbers);

    let converters: Vec<Vec<Map2>> = blocks.map(block_to_ranges).collect();

    let seed_ranges: Vec<Range<u32>> = seed_numbers
        .chunks_exact(2)
        .map(|n| n[0]..n[0] + n[1])
        .collect();

    // let a = seed_ranges
    //     .iter()
    //     .fold(0u64, |t, r| t + ((r.end - r.start) as u64));
    // println!("{a}");

    let lowest_location = seed_ranges
        .iter()
        .map(|range| {
            range
                .clone()
                .map(|seed| apply_conversions(&converters, seed))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("{lowest_location}");
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
