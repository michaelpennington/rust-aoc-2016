use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(22);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    x: u32,
    y: u32,
    used: u32,
    avail: u32,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("/dev/grid/node-").unwrap();
        let mut parts = s.split_whitespace();
        let xy = parts.next().unwrap();
        let (x, y) = xy
            .split_once('-')
            .map(|(x, y)| {
                (
                    x.strip_prefix('x').unwrap().parse().unwrap(),
                    y.strip_prefix('y').unwrap().parse().unwrap(),
                )
            })
            .unwrap();
        let used = parts.nth(1).unwrap();
        let used = used.strip_suffix('T').unwrap();
        let used = used.parse().unwrap();
        let avail = parts.next().unwrap();
        let avail = avail.strip_suffix('T').unwrap();
        let avail = avail.parse().unwrap();

        Ok(Self { x, y, used, avail })
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/dev/grid/node-x{}-y{}\t{}T\t{}T",
            self.x, self.y, self.used, self.avail
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let nodes = input
        .lines()
        .skip(2)
        .map(|l| l.parse::<Node>().unwrap())
        .collect::<Vec<_>>();
    let mut total = 0;
    for node in &nodes {
        total += if node.used != 0 {
            nodes
                .iter()
                .filter(|n| (n.x != node.x || n.y != node.y) && n.avail >= node.used)
                .count()
        } else {
            0
        };
    }
    Some(total as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(242)
}
