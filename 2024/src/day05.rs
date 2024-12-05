use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use anyhow::Ok;

pub const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn push_to_entry<K: Eq + Hash, V>(map: &mut HashMap<K, Vec<V>>, key: K, value: V) {
    map.entry(key).or_default().push(value);
}

fn middle<T: Sized>(of: &[T]) -> &T {
    &of[of.len() / 2]
}

fn add_to_set<T: Eq + Hash + Clone>(set: &mut HashSet<T>, list: &[T]) {
    for v in list {
        set.insert(v.clone());
    }
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut predecessors = HashMap::<i32, Vec<i32>>::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        match line.split_once('|') {
            Some((before, after)) => {
                push_to_entry(&mut predecessors, after.parse()?, before.parse()?);
            }
            None => break,
        }
    }

    let mut total = 0;
    for line in lines {
        let pages: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        let mut invalid_pages = HashSet::new();
        let mut ok = true;
        for current_page in pages.iter() {
            if invalid_pages.contains(current_page) {
                ok = false;
                break;
            }

            if let Some(predecessors) = predecessors.get(current_page) {
                add_to_set(&mut invalid_pages, predecessors);
            }
        }

        if ok {
            total += middle(&pages);
        }
    }

    Ok(format!("{total}"))
}

fn pages_that_need_some_swapping(
    pages: &[i32],
    predecessors: &HashMap<i32, Vec<i32>>,
) -> Option<Vec<(usize, usize)>> {
    let mut invalid_pages: Vec<(usize, i32)> = Vec::new();
    let mut swaps_to_do: Vec<(usize, usize)> = Vec::new();
    for (i, current_page) in pages.iter().enumerate() {
        if let Some((other_i, _)) = invalid_pages.iter().find(|(_, v)| v == current_page) {
            swaps_to_do.push((*other_i, i));
            break;
        }

        if let Some(predecessors) = predecessors.get(current_page) {
            for p in predecessors {
                invalid_pages.push((i, *p));
            }
        }
    }

    if swaps_to_do.is_empty() {
        None
    } else {
        Some(swaps_to_do)
    }
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut predecessors = HashMap::<i32, Vec<i32>>::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        match line.split_once('|') {
            Some((before, after)) => {
                push_to_entry(&mut predecessors, after.parse()?, before.parse()?);
            }
            None => break,
        }
    }

    let mut total = 0;
    for line in lines {
        let mut pages: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        let mut had_to_reorder = false;
        while let Some(pages_to_swap) = pages_that_need_some_swapping(&pages, &predecessors) {
            for (a, b) in pages_to_swap {
                pages.swap(a, b);
            }
            had_to_reorder = true;
        }

        if had_to_reorder {
            total += middle(&pages);
        }
    }

    Ok(format!("{total}"))
}
