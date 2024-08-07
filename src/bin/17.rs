use md5::{digest::generic_array::GenericArray, Digest, Md5};

use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(17);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dir {
    L,
    R,
    D,
    U,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::L => 'L',
                Dir::R => 'R',
                Dir::D => 'D',
                Dir::U => 'U',
            }
        )
    }
}

impl TryFrom<char> for Dir {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::L),
            'R' => Ok(Self::R),
            'U' => Ok(Self::U),
            'D' => Ok(Self::D),
            _ => Err("Invalid character"),
        }
    }
}

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Path(Vec<Dir>);

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl FromStr for Path {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Vec::with_capacity(s.len());
        for c in s.chars() {
            out.push(c.try_into()?);
        }
        Ok(Self(out))
    }
}

impl Path {
    fn loc(&self) -> (u8, u8) {
        let mut x = 0;
        let mut y = 0;
        for dir in &self.0 {
            match dir {
                Dir::L => x -= 1,
                Dir::R => x += 1,
                Dir::D => y += 1,
                Dir::U => y -= 1,
            }
        }
        (x, y)
    }
}

const DIRS: [Dir; 4] = [Dir::L, Dir::R, Dir::U, Dir::D];

pub fn part_one(input: &str) -> Option<Path> {
    let input = input.trim().as_bytes();
    let mut attempts: Vec<Path> = Vec::new();
    let mut winner: Option<Path> = None;
    let mut md5 = Md5::new();
    let mut output = GenericArray::default();
    md5.update(input);
    md5.finalize_into_reset(&mut output);
    let (x, y) = (0, 0);
    for dir in DIRS.iter().filter_map(|dir| match (dir, x, y) {
        (Dir::L, 0, _) | (Dir::R, 3, _) | (Dir::U, _, 0) | (Dir::D, _, 3) => None,
        (Dir::L, _, _) if (output[1] >> 4) > 10 => Some(Dir::L),
        (Dir::R, _, _) if (output[1] & 0b00001111) > 10 => Some(Dir::R),
        (Dir::U, _, _) if (output[0] >> 4) > 10 => Some(Dir::U),
        (Dir::D, _, _) if (output[0] & 0b00001111) > 10 => Some(Dir::D),
        _ => None,
    }) {
        let mut path = Path(Vec::new());
        path.0.push(dir);
        attempts.push(path);
    }
    while winner.is_none() {
        let mut new_attempts: Vec<Path> = Vec::new();
        for path in &attempts {
            md5.update(input);
            md5.update(path.to_string().as_bytes());
            md5.finalize_into_reset(&mut output);
            let (x, y) = path.loc();
            for (dir, x, y) in DIRS.iter().filter_map(|dir| match (dir, x, y) {
                (Dir::L, 0, _) | (Dir::R, 3, _) | (Dir::U, _, 0) | (Dir::D, _, 3) => None,
                (Dir::L, x, y) if (output[1] >> 4) > 10 => Some((Dir::L, x - 1, y)),
                (Dir::R, x, y) if (output[1] & 0b00001111) > 10 => Some((Dir::R, x + 1, y)),
                (Dir::U, x, y) if (output[0] >> 4) > 10 => Some((Dir::U, x, y - 1)),
                (Dir::D, x, y) if (output[0] & 0b00001111) > 10 => Some((Dir::D, x, y + 1)),
                _ => None,
            }) {
                let mut path = path.clone();
                path.0.push(dir);
                if x == 3 && y == 3 {
                    winner = Some(path);
                    break;
                } else {
                    new_attempts.push(path);
                }
            }
        }
        attempts = new_attempts;
    }
    winner
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.trim().as_bytes();
    let mut attempts: Vec<Path> = Vec::new();
    let mut md5 = Md5::new();
    let mut output = GenericArray::default();
    md5.update(input);
    md5.finalize_into_reset(&mut output);
    let (x, y) = (0, 0);
    for dir in DIRS.iter().filter_map(|dir| match (dir, x, y) {
        (Dir::L, 0, _) | (Dir::R, 3, _) | (Dir::U, _, 0) | (Dir::D, _, 3) => None,
        (Dir::L, _, _) if (output[1] >> 4) > 10 => Some(Dir::L),
        (Dir::R, _, _) if (output[1] & 0b00001111) > 10 => Some(Dir::R),
        (Dir::U, _, _) if (output[0] >> 4) > 10 => Some(Dir::U),
        (Dir::D, _, _) if (output[0] & 0b00001111) > 10 => Some(Dir::D),
        _ => None,
    }) {
        let mut path = Path(Vec::new());
        path.0.push(dir);
        attempts.push(path);
    }
    let mut last_final = None;
    loop {
        let mut new_attempts: Vec<Path> = Vec::new();
        for path in &attempts {
            md5.update(input);
            md5.update(path.to_string().as_bytes());
            md5.finalize_into_reset(&mut output);
            let (x, y) = path.loc();
            for (dir, x, y) in DIRS.iter().filter_map(|dir| match (dir, x, y) {
                (Dir::L, 0, _) | (Dir::R, 3, _) | (Dir::U, _, 0) | (Dir::D, _, 3) => None,
                (Dir::L, x, y) if (output[1] >> 4) > 10 => Some((Dir::L, x - 1, y)),
                (Dir::R, x, y) if (output[1] & 0b00001111) > 10 => Some((Dir::R, x + 1, y)),
                (Dir::U, x, y) if (output[0] >> 4) > 10 => Some((Dir::U, x, y - 1)),
                (Dir::D, x, y) if (output[0] & 0b00001111) > 10 => Some((Dir::D, x, y + 1)),
                _ => None,
            }) {
                let mut path = path.clone();
                path.0.push(dir);
                if x == 3 && y == 3 {
                    last_final = Some(path);
                    continue;
                } else {
                    new_attempts.push(path);
                }
            }
        }
        if let Some(ref p) = last_final {
            if new_attempts.is_empty() {
                return Some(p.0.len() as u32);
            }
        }
        attempts = new_attempts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("DDRRRD".parse().unwrap()));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("DDUDRLRRUDRD".parse().unwrap()));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(
            result,
            Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".parse().unwrap())
        );
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(370));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(492));
    }

    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(830));
    }
}
