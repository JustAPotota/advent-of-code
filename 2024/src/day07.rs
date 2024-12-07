use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum OperationPart2 {
    Add,
    Multiply,
    Concat,
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let sum: i64 = input
        .par_lines()
        .filter_map(|line| {
            let (test_value, equation) = line.split_once(':').unwrap();
            let test_value: i64 = test_value.parse().unwrap();
            let equation: Vec<i64> = equation
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let operation_sets = (0..equation.len() - 1)
                .map(|_| [Operation::Add, Operation::Multiply].into_iter())
                .multi_cartesian_product();

            for operations in operation_sets {
                let mut values = equation.clone().into_iter();
                let mut result = values.next().unwrap();
                for (value, operation) in values.zip(operations.into_iter()) {
                    match operation {
                        Operation::Add => result += value,
                        Operation::Multiply => result *= value,
                    }

                    if result > test_value {
                        break;
                    }
                }

                if result == test_value {
                    return Some(test_value);
                }
            }

            None
        })
        .sum();
    Ok(format!("{sum}"))
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let sum: i64 = input
        .par_lines()
        .filter_map(|line| {
            let (test_value, equation) = line.split_once(':').unwrap();
            let test_value: i64 = test_value.parse().unwrap();
            let equation: Vec<i64> = equation
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let operation_sets = (0..equation.len() - 1)
                .map(|_| {
                    [
                        OperationPart2::Add,
                        OperationPart2::Multiply,
                        OperationPart2::Concat,
                    ]
                    .into_iter()
                })
                .multi_cartesian_product();

            for operations in operation_sets {
                let mut values = equation.clone().into_iter();
                let mut result = values.next().unwrap();
                for (value, operation) in values.zip(operations.into_iter()) {
                    match operation {
                        OperationPart2::Add => result += value,
                        OperationPart2::Multiply => result *= value,
                        OperationPart2::Concat => {
                            result = format!("{result}{value}").parse().unwrap();
                        }
                    }

                    if result > test_value {
                        break;
                    }
                }

                if result == test_value {
                    return Some(test_value);
                }
            }

            None
        })
        .sum();
    Ok(format!("{sum}"))
}
