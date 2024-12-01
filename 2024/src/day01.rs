use std::collections::HashMap;

pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut list_a = Vec::<i32>::new();
    let mut list_b = Vec::<i32>::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        list_a.push(a.parse()?);
        list_b.push(b.parse()?);
    }

    assert_eq!(list_a.len(), list_b.len());

    list_a.sort();
    list_b.sort();

    let combined = list_a.into_iter().zip(list_b);
    let total = combined.fold(0u64, |total, (a, b)| a.abs_diff(b) as u64 + total);

    Ok(format!("{total}"))
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut list_a = Vec::<i32>::new();
    let mut list_b = HashMap::<i32, i32>::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();

        list_a.push(a.parse()?);
        *list_b.entry(b.parse()?).or_default() += 1;
    }

    let total = list_a
        .into_iter()
        .fold(0, |total, n| n * list_b.get(&n).unwrap_or(&0) + total);

    Ok(format!("{total}"))
}
