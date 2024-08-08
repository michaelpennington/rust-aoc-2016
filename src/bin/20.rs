use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(20);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Range {
    begin: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (begin, end) = s
            .split_once('-')
            .ok_or(format!("{s} doesn't contain '-'"))
            .and_then(|(first, second)| {
                first
                    .parse()
                    .map_err(|e| format!("Invalid u32: {e:?}"))
                    .and_then(|first| {
                        second
                            .parse()
                            .map_err(|e| format!("Invalid u32: {e:?}"))
                            .map(|second| (first, second))
                    })
            })?;
        Ok(Self { begin, end })
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.begin, self.end)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ranges = input
        .lines()
        .map(|l| l.parse::<Range>().unwrap())
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.begin);
    for range in &mut ranges {
        range.end = range.end.saturating_add(1);
    }
    let mut index = 0;
    for i in 1..ranges.len() {
        if ranges[index].end >= ranges[i].begin {
            ranges[index].end = ranges[index].end.max(ranges[i].end);
        } else {
            index += 1;
            ranges[index] = ranges[i];
        }
    }
    ranges.truncate(index + 1);
    ranges.first().map(|r| r.end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ranges = input
        .lines()
        .map(|l| l.parse::<Range>().unwrap())
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.begin);
    for range in &mut ranges {
        range.end = range.end.saturating_add(1);
    }
    let mut index = 0;
    for i in 1..ranges.len() {
        if ranges[index].end >= ranges[i].begin {
            ranges[index].end = ranges[index].end.max(ranges[i].end);
        } else {
            index += 1;
            ranges[index] = ranges[i];
        }
    }
    ranges.truncate(index + 1);
    Some(ranges.windows(2).map(|w| w[1].begin - w[0].end).sum())
}
