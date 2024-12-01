use std::time::{Duration, Instant};

pub fn time_fn<F, I, O>(f: F, input: I) -> (O, Duration)
where
    F: FnOnce(I) -> O,
{
    let start_time = Instant::now();
    let output = f(input);
    (output, start_time.elapsed())
}

pub fn main(input: &str, part1: fn(&str) -> u32, part2: fn(&str) -> u32) {
    let args: Vec<String> = std::env::args().collect();
    let part: u8 = args.get(1).unwrap().parse().unwrap();
    if part == 1 {
        let (output, time_taken) = time_fn(part1, input);
        println!("Got '{output}' in {time_taken:#?}");
    } else if part == 2 {
        let (output, time_taken) = time_fn(part2, input);
        println!("Got '{output}' in {time_taken:#?}");
    }
}
