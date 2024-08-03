use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    ops::{Add, Index, IndexMut},
};

advent_of_code::solution!(13);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Open,
    Wall,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Open => write!(f, "."),
            Space::Wall => write!(f, "#"),
        }
    }
}

const MAP_SIZE: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Map {
    spaces: [[Space; MAP_SIZE]; MAP_SIZE],
}

impl Default for Map {
    fn default() -> Self {
        Self {
            spaces: [[Space::Open; MAP_SIZE]; MAP_SIZE],
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.spaces {
            for ch in line {
                write!(f, "{ch}")?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

const START: (usize, usize) = (1, 1);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Node {
    pt: (usize, usize),
    score: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.score).cmp(&Reverse(other.score))
    }
}

impl Map {
    fn new_from_seed(seed: usize) -> Self {
        let mut out = Self::default();
        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                if (x * x + 3 * x + 2 * x * y + y + y * y + seed).count_ones() % 2 == 1 {
                    out[(x, y)] = Space::Wall;
                }
            }
        }
        out
    }

    fn a_star(&self, target: (usize, usize)) -> Option<u32> {
        let h = |from| h(from, target);
        let mut open_set = BinaryHeap::new();
        open_set.push(Node {
            pt: START,
            score: 0,
        });
        // let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(START, 0);
        let mut f_score = HashMap::new();
        f_score.insert(START, h(START));
        while let Some(current) = open_set.pop() {
            if current.pt == target {
                return Some(current.score);
            }
            for neighbor in self.neighbors(current.pt) {
                let tentative_g_score = g_score[&current.pt] + 1;
                if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(u32::MAX) {
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + h(neighbor));
                    open_set.push(Node {
                        pt: neighbor,
                        score: tentative_g_score,
                    });
                }
            }
        }
        None
    }

    fn neighbors(&self, pt: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        (pt.0.saturating_sub(1)..=pt.0.add(1).min(MAP_SIZE - 1))
            .flat_map(move |x| {
                (pt.1.saturating_sub(1)..=pt.1.add(1).min(MAP_SIZE - 1)).map(move |y| (x, y))
            })
            .filter(move |&(x, y)| (x == pt.0 && y != pt.1) || (x != pt.0 && y == pt.1))
            .filter(|pt| self[*pt] == Space::Open)
    }
}

fn h(from: (usize, usize), to: (usize, usize)) -> u32 {
    (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as u32
}

impl Index<(usize, usize)> for Map {
    type Output = Space;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.spaces[index.1][index.0]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.spaces[index.1][index.0]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let seed = lines.next()?.parse().unwrap();
    let map = Map::new_from_seed(seed);
    let target: (usize, usize) = lines.next().and_then(|l| {
        l.trim()
            .split_once(',')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
    })?;
    map.a_star(target)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let seed = lines.next()?.parse().unwrap();
    let map = Map::new_from_seed(seed);
    Some(
        (0..MAP_SIZE)
            .flat_map(|x| (0..MAP_SIZE).map(move |y| (x, y)))
            .filter(|&tgt| map.a_star(tgt).unwrap_or(51) <= 50)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }
}
