use regex::Regex;

pub fn part1(input: &str) -> anyhow::Result<String> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let mut sum = 0;
    for (_, [a, b]) in regex.captures_iter(input).map(|c| c.extract()) {
        let a: i32 = a.parse()?;
        let b: i32 = b.parse()?;
        sum += a * b;
    }

    Ok(format!("{sum}"))
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let regex = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))")?;

    let mut sum = 0;
    let mut enabled = true;
    for captures in regex.captures_iter(input) {
        if captures.get(1).is_some() && enabled {
            let a: i32 = captures.get(2).unwrap().as_str().parse()?;
            let b: i32 = captures.get(3).unwrap().as_str().parse()?;
            sum += a * b;
        } else if captures.get(4).is_some() {
            enabled = true;
        } else if captures.get(5).is_some() {
            enabled = false;
        }
    }

    Ok(format!("{sum}"))
}
