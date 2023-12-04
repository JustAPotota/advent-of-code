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

fn get_numbers(line: &str) -> (Vec<u16>, Vec<u16>) {
    let (_, line) = line.split_once(':').unwrap();
    let (winning_side, your_side) = line.split_once(" |").unwrap();
    let winning_numbers: Vec<u16> = winning_side
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let your_numbers: Vec<u16> = your_side
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    (winning_numbers, your_numbers)
}

fn get_score(line: &str) -> usize {
    let (winning_numbers, your_numbers) = get_numbers(line);

    let score = your_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();
    score
}

fn part1(input: &str) {
    let total_score = input.lines().fold(0, |total_score, line| {
        let score = get_score(line);

        if score == 0 {
            total_score
        } else {
            total_score + 2u16.pow((score - 1) as u32)
        }
    });
    println!("{total_score}");
}

fn part2(input: &str) {
    let card_count = input.lines().count();
    let mut card_multipliers = vec![0; card_count];
    for (card_number, line) in input.lines().enumerate() {
        card_multipliers[card_number] += 1;

        let score = get_score(line);
        if score == 0 {
            continue;
        }

        let this_multiplier = card_multipliers[card_number];
        let next_card_index = card_number + 1;
        for multiplier in card_multipliers[next_card_index..next_card_index + score].iter_mut() {
            *multiplier += this_multiplier;
        }
    }
    println!("{}", card_multipliers.iter().sum::<usize>());
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
