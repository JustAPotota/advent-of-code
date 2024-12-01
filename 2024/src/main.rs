#![feature(concat_idents)]

use std::{io::Read, time::Instant};

use clap::{value_parser, Parser};
use clio::Input;

macro_rules! import_days {
    ($($d:expr),*) => {
        $(
            ::paste::paste! {
                mod [<day $d>];
            }
        )*
        const DAYS: &[(fn(&str) -> ::anyhow::Result<String>, fn(&str) -> ::anyhow::Result<String>)] = &[
            $(::paste::paste! {
                ([<day $d>]::part1, [<day $d>]::part2)
            },)*
        ];
    };
}

import_days!(00, 01);

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, value_parser = value_parser!(u8).range(1..=2))]
    part: u8,

    #[arg(short, long, value_parser)]
    input: Input,
}

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();

    let mut input = String::new();
    args.input.read_to_string(&mut input)?;

    match DAYS.get(args.day as usize) {
        Some(program) => {
            let start = Instant::now();
            let output = match args.part {
                1 => program.0,
                2 => program.1,
                _ => panic!(),
            }(&input);
            let time = start.elapsed();
            println!("{:?}, Took {}s", output, time.as_secs_f64());
        }
        None => panic!("no program for day #{}", args.day),
    };

    Ok(())
}
