use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, AddAssign},
};

use bitvec::vec::BitVec;
use rayon::prelude::*;

pub const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
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
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn position_offset(&self) -> Position {
        match self {
            Self::Up => Position::new(0, -1),
            Self::Down => Position::new(0, 1),
            Self::Left => Position::new(-1, 0),
            Self::Right => Position::new(1, 0),
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

fn parse_input(input: &str) -> anyhow::Result<(Guard, Vec<BitVec>)> {
    let mut guard = Guard::default();

    let mut obstacles = Vec::<BitVec>::new();
    for (y, line) in input.lines().enumerate() {
        obstacles.push(BitVec::repeat(false, line.len()));
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                *obstacles[y].get_mut(x).unwrap() = true;
            }

            if char == '^' {
                guard.position = (x as i32, y as i32).into();
            }
        }
    }

    Ok((guard, obstacles))
}

fn visited_positions(mut guard: Guard, obstacles: &[BitVec]) -> HashSet<Position> {
    let y_bounds = 0..obstacles.len();
    let x_bounds = 0..obstacles[0].len();
    let mut visited = HashSet::<Position>::new();
    loop {
        visited.insert(guard.position);
        let next_position = guard.position + guard.direction.position_offset();

        if !x_bounds.contains(&(next_position.x as usize))
            || !y_bounds.contains(&(next_position.y as usize))
        {
            break;
        }

        if obstacles[next_position.y as usize][next_position.x as usize] {
            guard.direction = guard.direction.rotate_right();
        }
        guard.position += guard.direction.position_offset();
    }

    visited
}

fn does_guard_loop_with_new_obstacle(
    mut guard: Guard,
    mut obstacles: Vec<BitVec>,
    new_obstacle: Position,
) -> bool {
    let y_bounds = 0..obstacles.len();
    let x_bounds = 0..obstacles[0].len();

    if obstacles[new_obstacle.y as usize][new_obstacle.x as usize] {
        return false;
    }

    *obstacles[new_obstacle.y as usize]
        .get_mut(new_obstacle.x as usize)
        .unwrap() = true;

    let mut previous_collisions = HashSet::<(Position, Direction)>::new();

    loop {
        let next_position = guard.position + guard.direction.position_offset();

        if !x_bounds.contains(&(next_position.x as usize))
            || !y_bounds.contains(&(next_position.y as usize))
        {
            return false;
        }

        if obstacles[next_position.y as usize][next_position.x as usize] {
            if !previous_collisions.insert((guard.position, guard.direction)) {
                // println!("-----------------------------");
                // println!("{}", map_to_string(&obstacles));
                return true;
            }

            guard.direction = guard.direction.rotate_right();
        }
        guard.position += guard.direction.position_offset();
    }
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let (guard, obstacles) = parse_input(input)?;
    let visited = visited_positions(guard, &obstacles);

    Ok(format!("{}", visited.len()))
}

fn map_to_string(obstacles: &[BitVec]) -> String {
    obstacles
        .iter()
        .map(|row| {
            let mut s = row
                .iter()
                .map(|cell| if *cell { '#' } else { '.' })
                .collect::<String>();
            s.push('\n');
            s
        })
        .fold(String::new(), |mut s, line| {
            s.push_str(&line);
            s
        })
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let (guard, obstacles) = parse_input(input)?;

    let mut positions_to_try = visited_positions(guard.clone(), &obstacles);
    positions_to_try.remove(&guard.position);

    let positions_that_loop: HashSet<Position> = positions_to_try
        .into_par_iter()
        .filter(|new_obstacle| {
            does_guard_loop_with_new_obstacle(guard.clone(), obstacles.clone(), *new_obstacle)
        })
        .collect();

    Ok(format!("{}", positions_that_loop.len()))
}
