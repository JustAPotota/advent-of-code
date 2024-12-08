use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Range, Sub},
};

use itertools::Itertools;

pub const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub for &Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_in_bounds(&self, bounds: &(Range<i32>, Range<i32>)) -> bool {
        bounds.0.contains(&self.x) && bounds.1.contains(&self.y)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

fn find_antennas(map: &str) -> HashMap<char, Vec<Position>> {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    for (y, line) in map.lines().enumerate() {
        for (x, char) in line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_alphanumeric())
        {
            antennas
                .entry(char)
                .and_modify(|v| v.push((x as i32, y as i32).into()))
                .or_insert(vec![(x as i32, y as i32).into()]);
        }
    }

    antennas
}

fn antinodes_of_antennas(antennas: &[Position]) -> Vec<Position> {
    antennas
        .iter()
        .cartesian_product(antennas.iter())
        .filter_map(|(a, b)| {
            if a == b {
                None
            } else {
                let distance = a - b;
                Some(*a + distance)
            }

            // println!("antinode {} for antennas {} {}", *a + distance, a, b);
        })
        .collect()
}

fn harmonic_antinodes(antennas: &[Position], bounds: &(Range<i32>, Range<i32>)) -> Vec<Position> {
    antennas
        .iter()
        .cartesian_product(antennas.iter())
        .flat_map(|(a, b)| {
            if a == b {
                Vec::new()
            } else {
                let distance = a - b;
                let mut antinodes = Vec::new();

                let mut antinode = *b + distance;
                while antinode.is_in_bounds(bounds) {
                    let next_antinode = antinode + distance;
                    antinodes.push(antinode);
                    antinode = next_antinode;
                }

                antinodes
            }
        })
        .collect()
}

fn bounds(map: &str) -> (Range<i32>, Range<i32>) {
    let lines = map.lines().collect::<Vec<_>>();
    (0..lines[0].len() as i32, 0..lines.len() as i32)
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let bounds = bounds(input);
    let antennas = find_antennas(input);
    let antinodes: HashSet<Position> = antennas
        .into_iter()
        .flat_map(|(_, antennas)| antinodes_of_antennas(&antennas))
        .filter(|pos| pos.is_in_bounds(&bounds))
        .collect();
    Ok(format!("{}", antinodes.len()))
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let bounds = bounds(input);
    let antennas = find_antennas(input);
    let antinodes: HashSet<Position> = antennas
        .into_iter()
        .flat_map(|(_, antennas)| harmonic_antinodes(&antennas, &bounds))
        .collect();
    Ok(format!("{}", antinodes.len()))
}
