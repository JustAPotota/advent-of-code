use std::slice::SliceIndex;

pub const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

struct Grid {
    grid: Vec<char>,
    line_width: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let line_width = input.lines().next().unwrap().len() + 1;
        let grid = input.chars().collect::<Vec<_>>();
        Self { grid, line_width }
    }

    fn right(&self, from: usize, by: usize) -> usize {
        from + by
    }

    fn can_move_up(&self, from: usize, by: usize) -> bool {
        from >= by * self.line_width
    }

    fn up(&self, from: usize, by: usize) -> usize {
        from - by * self.line_width
    }

    fn can_move_left(&self, from: usize, by: usize) -> bool {
        from >= by
    }

    fn left(&self, from: usize, by: usize) -> usize {
        from - by
    }

    fn can_move_up_left(&self, from: usize, by: usize) -> bool {
        self.can_move_up(from, by) && self.can_move_left(self.left(from, by), by)
    }

    fn up_left(&self, from: usize, by: usize) -> usize {
        self.left(self.up(from, by), by)
    }

    fn can_move_up_right(&self, from: usize, by: usize) -> bool {
        self.can_move_up(from, by)
    }

    fn up_right(&self, from: usize, by: usize) -> usize {
        self.right(self.up(from, by), by)
    }

    fn get<I: SliceIndex<[char]>>(&self, position: I) -> Option<&I::Output> {
        self.grid.get(position)
    }
}

fn search_right(grid: &Grid, word: &[char], at: usize) -> bool {
    grid.get(at..at + word.len()).is_some_and(|str| str == word)
}

fn search_down(grid: &Grid, word: &[char], at: usize) -> bool {
    for (i, char) in word.iter().enumerate() {
        let position = i * grid.line_width + at;
        if grid.get(position).is_none_or(|c| c != char) {
            return false;
        }
    }
    true
}

fn search_down_right(grid: &Grid, word: &[char], at: usize) -> bool {
    for (i, char) in word.iter().enumerate() {
        let position = i * grid.line_width + at + i;
        if grid.get(position).is_none_or(|c| c != char) {
            return false;
        }
    }
    true
}

fn search_down_left(grid: &Grid, word: &[char], at: usize) -> bool {
    for (i, char) in word.iter().enumerate() {
        let position = i * grid.line_width + at - i;
        if grid.get(position).is_none_or(|c| c != char) {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    const WORD: &[char] = &['X', 'M', 'A', 'S'];
    let mut reversed = WORD.to_owned();
    reversed.reverse();

    let word_len = WORD.len();
    let grid = Grid::new(input);

    let mut count = 0;
    for (i, char) in grid.grid.iter().enumerate() {
        if *char != WORD[0] {
            continue;
        }
        if search_right(&grid, WORD, i) {
            // Right
            // println!("found right xmas at {i}");
            count += 1;
        }
        if grid.can_move_left(i, word_len - 1)
            && search_right(&grid, &reversed, grid.left(i, word_len - 1))
        {
            // Left
            // println!("found left xmas at {i}");
            count += 1;
        }

        if search_down(&grid, WORD, i) {
            // Down
            // println!("found down xmas at {i}");
            count += 1;
        }
        if grid.can_move_up(i, word_len - 1)
            && search_down(&grid, &reversed, grid.up(i, word_len - 1))
        {
            // Up
            // println!("found up xmas at {i}");
            count += 1;
        }

        if search_down_right(&grid, WORD, i) {
            // Down right
            // println!("found down right xmas at {i}");
            count += 1;
        }
        if grid.can_move_up_left(i, word_len - 1)
            && search_down_right(&grid, &reversed, grid.up_left(i, word_len - 1))
        {
            // Up left
            // println!("found up left xmas at {i}");
            count += 1;
        }

        if search_down_left(&grid, WORD, i) {
            // Down left
            // println!("found down left xmas at {i}");
            count += 1;
        }
        if grid.can_move_up_right(i, word_len - 1)
            && search_down_left(&grid, &reversed, grid.up_right(i, word_len - 1))
        {
            // Up right
            // println!("found up right xmas at {i}");
            count += 1;
        }
    }

    Ok(format!("{count}"))
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let grid = Grid::new(input);

    let mut count = 0;
    let valid_center_positions = grid.line_width - 1..input.len() - grid.line_width - 1;
    for (i, _) in input.chars().enumerate().filter(|(_, c)| *c == 'A') {
        if !valid_center_positions.contains(&i) {
            continue;
        }

        let corners = [
            grid.grid[i - grid.line_width - 1],
            grid.grid[i - grid.line_width + 1],
            grid.grid[i + grid.line_width - 1],
            grid.grid[i + grid.line_width + 1],
        ];

        if !corners.iter().all(|c| *c == 'M' || *c == 'S') {
            continue;
        }

        if (corners[0] == corners[2] && corners[1] == corners[3] && corners[0] != corners[1])
            || (corners[0] == corners[1] && corners[2] == corners[3] && corners[0] != corners[2])
        {
            count += 1;
        }
    }

    Ok(format!("{count}"))
}

// valid: SMSM, MSMS, MMSS, SSMM (XYXY, XXYY)
// invalid: SMMS, MSSM
/*
S-M M-S  M-M S-S
-A- -A-  -A- -A-
S-M M-S  S-S M-M

*/
