use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    ops::{Add, Index},
    str::FromStr,
};

use itertools::Itertools;

use anyhow::{anyhow, ensure};

advent_of_code::solution!(24);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Open,
    Wall,
}

impl TryFrom<char> for Space {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | '0'..='9' => Ok(Self::Open),
            '#' => Ok(Self::Wall),
            _ => Err(anyhow!("{value} is not a valid map character")),
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::Open => '.',
                Space::Wall => '#',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    loc: (usize, usize),
    num: u8,
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Space>>,
    nodes: Vec<Node>,
    node_distances: HashMap<(u8, u8), u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct N {
    loc: (usize, usize),
    score: u32,
}

impl Ord for N {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.score).cmp(&Reverse(other.score))
    }
}

impl PartialOrd for N {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Space;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.map[index.1][index.0]
    }
}

fn h(a: (usize, usize), b: (usize, usize)) -> u32 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u32
}

impl Map {
    fn a_star(&self, from: (usize, usize), to: (usize, usize)) -> u32 {
        let h = |from| h(from, to);
        let mut open_set = BinaryHeap::new();
        open_set.push(N {
            loc: from,
            score: 0,
        });

        let mut g_score = HashMap::new();
        g_score.insert(from, 0);

        let mut f_score = HashMap::new();
        f_score.insert(from, h(from));
        while let Some(current) = open_set.pop() {
            if current.loc == to {
                return current.score;
            }
            for neighbor in self.neighbors(current.loc) {
                let tentative_g_score = g_score[&current.loc] + 1;
                if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(u32::MAX) {
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + h(neighbor));
                    open_set.push(N {
                        loc: neighbor,
                        score: tentative_g_score,
                    });
                }
            }
        }
        u32::MAX
    }

    fn neighbors(&self, pt: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        (pt.0.saturating_sub(1)..pt.0.add(2).min(self.map[0].len())).flat_map(move |x| {
            (pt.1.saturating_sub(1)..pt.1.add(2).min(self.map.len()))
                .map(move |y| (x, y))
                .filter(move |&(x, y)| (x == pt.0 && y != pt.1) || (x != pt.0 && y == pt.1))
                .filter(|&p| self[p] == Space::Open)
        })
    }

    fn calc_distance(&mut self, node1: u8, node2: u8) {
        let n1 = self
            .nodes
            .iter()
            .find_map(|n| (n.num == node1).then_some(n.loc))
            .expect("Not a valid node");
        let n2 = self
            .nodes
            .iter()
            .find_map(|n| (n.num == node2).then_some(n.loc))
            .expect("Not a valid node");
        let d = self.a_star(n1, n2);
        self.node_distances.insert((node1, node2), d);
        self.node_distances.insert((node2, node1), d);
    }

    fn calc_all(&mut self) {
        let nodes = self.nodes.clone();
        for (i, n1) in nodes.iter().enumerate() {
            for n2 in nodes[i + 1..].iter() {
                self.calc_distance(n1.num, n2.num);
            }
        }
    }

    fn find_shortest(&mut self) -> Option<u32> {
        self.calc_all();
        (1..self.nodes.len() as u8)
            .permutations(self.nodes.len() - 1)
            .map(|perm| std::iter::once(0).chain(perm))
            .map(|perm| {
                perm.tuple_windows()
                    .map(|t| self.node_distances[&t])
                    .sum::<u32>()
            })
            .min()
    }

    fn find_shortest0(&mut self) -> Option<u32> {
        self.calc_all();
        (1..self.nodes.len() as u8)
            .permutations(self.nodes.len() - 1)
            .map(|perm| std::iter::once(0).chain(perm).chain(std::iter::once(0)))
            .map(|perm| {
                perm.tuple_windows()
                    .map(|t| self.node_distances[&t])
                    .sum::<u32>()
            })
            .min()
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::with_capacity(s.lines().count());
        let mut nodes = Vec::new();
        for (y, line) in s.lines().enumerate() {
            let mut l = Vec::with_capacity(line.len());
            for (x, ch) in line.chars().enumerate() {
                l.push(ch.try_into()?);
                if ch.is_ascii_digit() {
                    let num = match ch {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        _ => unreachable!("Already checked ch is an ascii digit"),
                    };
                    nodes.push(Node { loc: (x, y), num });
                }
            }
            map.push(l);
        }
        let node_distances = HashMap::with_capacity(nodes.len() * nodes.len());
        let line_len = map[0].len();
        ensure!(
            map.iter().all(|l| l.len() == line_len),
            "Lines must all be the same length"
        );
        Ok(Self {
            map,
            nodes,
            node_distances,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.map.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if let Some(n) = self
                    .nodes
                    .iter()
                    .find_map(|n| (n.loc == (x, y)).then_some(n.num))
                {
                    write!(f, "{n}")?;
                } else {
                    write!(f, "{ch}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input.parse::<Map>().unwrap();
    map.find_shortest()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = input.parse::<Map>().unwrap();
    map.find_shortest0()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20));
    }
}
