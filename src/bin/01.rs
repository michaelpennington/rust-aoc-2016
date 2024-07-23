use std::{collections::HashSet, ops::AddAssign, str::FromStr};

advent_of_code::solution!(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    len: i32,
    turn: Turn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Turn {
    R,
    L,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Loc {
    x: i32,
    y: i32,
    dir: Dir,
}

impl Default for Loc {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Dir::N,
        }
    }
}

impl Loc {
    const fn taxicab_len(&self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }

    fn process(&mut self, instructions: &[Instruction]) -> u32 {
        for inst in instructions {
            *self += inst;
        }
        self.taxicab_len()
    }

    fn processv2(&mut self, instructions: &[Instruction]) -> u32 {
        let mut points_seen = HashSet::with_capacity(instructions.len());
        for inst in instructions {
            let mut dir = self.dir;
            dir += inst.turn;
            match dir {
                Dir::N => {
                    for y in 0..(inst.len) {
                        if !points_seen.insert((self.x, self.y + y)) {
                            return self.x.unsigned_abs() + (self.y + y).unsigned_abs();
                        }
                    }
                }
                Dir::S => {
                    for y in 0..(inst.len) {
                        if !points_seen.insert((self.x, self.y - y)) {
                            return self.x.unsigned_abs() + (self.y - y).unsigned_abs();
                        }
                    }
                }
                Dir::E => {
                    for x in 0..(inst.len) {
                        if !points_seen.insert((self.x + x, self.y)) {
                            return (self.x + x).unsigned_abs() + self.y.unsigned_abs();
                        }
                    }
                }
                Dir::W => {
                    for x in 0..(inst.len) {
                        if !points_seen.insert((self.x - x, self.y)) {
                            return (self.x - x).unsigned_abs() + self.y.unsigned_abs();
                        }
                    }
                }
            }
            *self += inst;
        }
        0
    }
}

impl AddAssign<&Instruction> for Loc {
    fn add_assign(&mut self, rhs: &Instruction) {
        self.dir += rhs.turn;
        match self.dir {
            Dir::N => self.y += rhs.len,
            Dir::S => self.y -= rhs.len,
            Dir::E => self.x += rhs.len,
            Dir::W => self.x -= rhs.len,
        }
    }
}

impl AddAssign<Turn> for Dir {
    fn add_assign(&mut self, rhs: Turn) {
        match (*self, rhs) {
            (Dir::N, Turn::R) | (Dir::S, Turn::L) => *self = Dir::E,
            (Dir::N, Turn::L) | (Dir::S, Turn::R) => *self = Dir::W,
            (Dir::E, Turn::R) | (Dir::W, Turn::L) => *self = Dir::S,
            (Dir::E, Turn::L) | (Dir::W, Turn::R) => *self = Dir::N,
        }
    }
}

impl FromStr for Turn {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Turn::L),
            "R" => Ok(Turn::R),
            _ => Err(()),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (turn, len) = s.split_at(1);
        let turn = turn.parse()?;
        let len = len.parse().map_err(|_| ())?;
        Ok(Self { turn, len })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    N,
    S,
    E,
    W,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut loc = Loc::default();
    let directions = input
        .trim()
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    Some(loc.process(&directions))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut loc = Loc::default();
    let directions = input
        .trim()
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    Some(loc.processv2(&directions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));
    }
}
