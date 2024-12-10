#![feature(concat_idents)]

use std::{io::Read, time::Instant};

use clap::{value_parser, Parser};
use clio::Input;

type Program = fn(&str) -> anyhow::Result<String>;
struct Day {
    part1: Program,
    part2: Program,
    test_str: &'static str,
}

macro_rules! import_days {
    ($($d:expr),*) => {
        ::paste::paste! {
            $(
                mod [<day $d>];
            )*
            const DAYS: &[Day] = &[
                $(Day{part1:[<day $d>]::part1,part2:[<day $d>]::part2,test_str:[<day $d>]::TEST},)*
            ];
        }
    };
}

import_days!(00, 01, 02, 03, 04, 05, 06, 07, 08, 09);

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, value_parser = value_parser!(u8).range(1..=2))]
    part: u8,

    #[arg(short, long, value_parser)]
    input: Option<Input>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match DAYS.get(args.day as usize) {
        Some(day) => {
            let start = Instant::now();
            let input = {
                match args.input {
                    Some(mut input_file) => {
                        let mut input = String::new();
                        input_file.read_to_string(&mut input)?;
                        input
                    }
                    None => day.test_str.to_owned(),
                }
            };

            let output = match args.part {
                1 => day.part1,
                2 => day.part2,
                _ => panic!(),
            }(&input);

            let time = start.elapsed();
            println!("{:?}, Took {}s", output, time.as_secs_f64());
        }
        None => panic!("no program for day #{}", args.day),
    };

    Ok(())
}
