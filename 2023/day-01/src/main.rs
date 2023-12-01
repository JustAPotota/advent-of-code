use std::{collections::HashMap, fs};

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let out = input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
            let (a, b) = (digits.first().unwrap(), digits.last().unwrap());
            let s = format!("{a}{b}");

            let n = u32::from_str_radix(&s, 10).unwrap();
            println!("{n}");
            n
        })
        .fold(0, |total, s| total + s);
    println!("{out}");
}

fn part2() {
    let number_names = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let input = fs::read_to_string("input.txt").unwrap();
    let out = input
        .lines()
        .map(|line| {
            let mut char_iter = line.chars().enumerate();
            let mut digits = Vec::new();
            while let Some((i, ch)) = char_iter.next() {
                if ch.is_numeric() {
                    digits.push(ch);
                    continue;
                }
                for (name, digit) in number_names.iter() {
                    let name_len = name.len();
                    if line.len() >= name_len + i && line[i..i + name_len].find(name).is_some() {
                        digits.push(*digit);
                    }
                }
            }
            println!("{line} {digits:?}");
            let (a, b) = (digits.first().unwrap(), digits.last().unwrap());
            let s = format!("{a}{b}");

            let n = u32::from_str_radix(&s, 10).unwrap();
            println!("{n}");
            n
        })
        .fold(0, |total, s| total + s);
    println!("{out}");
}

fn main() {
    //part1();
    part2();
}
