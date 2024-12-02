use std::cell::OnceCell;

fn is_report_safe(report: &[i32]) -> bool {
    let mut report_ascending = None;
    for pair in report.windows(2) {
        let a = pair[0];
        let b = pair[1];

        let diff = a - b;

        if !(1..4).contains(&diff.abs()) {
            return false;
        }

        let this_ascending = diff >= 0;

        match report_ascending {
            Some(report_ascending) => {
                if report_ascending != this_ascending {
                    return false;
                }
            }
            None => report_ascending = Some(this_ascending),
        }
    }

    true
}

fn is_report_safe_dampened(report: &[i32]) -> bool {
    let normally_safe = is_report_safe(report);
    if normally_safe {
        return true;
    }

    for i in 0..report.len() {
        let mut short_report = report.to_vec();
        short_report.remove(i);
        if is_report_safe(&short_report) {
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let safe_reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|report| is_report_safe(&report))
        .filter(|b| *b)
        .count();

    Ok(format!("{safe_reports}"))
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let safe_reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|report| is_report_safe_dampened(&report))
        .filter(|b| *b)
        .count();

    Ok(format!("{safe_reports}"))
}
