use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(21);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Password(Vec<char>);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl<T> From<T> for Password
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self(value.as_ref().chars().collect())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Command {
    Swap(usize, usize),
    SwapC(char, char),
    RotateL(usize),
    RotateR(usize),
    RotateC(char),
    Reverse(usize, usize),
    RotateRev(char),
    Move(usize, usize),
}

impl Command {
    fn reverse(self) -> Self {
        match self {
            Self::RotateL(s) => Self::RotateR(s),
            Self::RotateR(s) => Self::RotateL(s),
            Self::RotateC(c) => Self::RotateRev(c),
            Self::Move(x, y) => Self::Move(y, x),
            _ => self,
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let err = || format!("Not parsable {s}");
        match words.next().unwrap() {
            "rotate" => match words.next().unwrap() {
                "left" => {
                    let steps = words.next().unwrap().parse().map_err(|_| err())?;
                    Ok(Self::RotateL(steps))
                }
                "right" => {
                    let steps = words.next().unwrap().parse().map_err(|_| err())?;
                    Ok(Self::RotateR(steps))
                }
                "based" => {
                    let c = words.nth(4).unwrap().chars().next().unwrap();
                    Ok(Self::RotateC(c))
                }
                _ => Err(err()),
            },
            "swap" => match words.next().unwrap() {
                "letter" => {
                    let a = words.next().unwrap().chars().next().unwrap();
                    let b = words.nth(2).unwrap().chars().next().unwrap();
                    Ok(Self::SwapC(a, b))
                }
                "position" => {
                    let x = words.next().unwrap().parse().unwrap();
                    let y = words.nth(2).unwrap().parse().unwrap();
                    Ok(Self::Swap(x, y))
                }
                _ => Err(err()),
            },
            "reverse" => {
                let x = words.nth(1).unwrap().parse().unwrap();
                let y = words.nth(1).unwrap().parse().unwrap();
                Ok(Self::Reverse(x, y))
            }
            "move" => {
                let x = words.nth(1).unwrap().parse().unwrap();
                let y = words.nth(2).unwrap().parse().unwrap();
                Ok(Self::Move(x, y))
            }
            _ => Err(err()),
        }
    }
}

impl Password {
    fn position(&self, c: char) -> usize {
        self.0
            .iter()
            .position(|&a| a == c)
            .expect("inputs should match")
    }

    fn swap(&mut self, x: usize, y: usize) {
        self.0.swap(x, y);
    }

    fn swap_c(&mut self, a: char, b: char) {
        let x = self.position(a);
        let y = self.position(b);
        self.swap(x, y);
    }

    fn rotate_l(&mut self, steps: usize) {
        self.0.rotate_left(steps);
    }

    fn rotate_r(&mut self, steps: usize) {
        self.0.rotate_right(steps);
    }

    fn rotate_c(&mut self, c: char) {
        let p = self.position(c);
        let n_times = if p >= 4 { p + 2 } else { p + 1 };
        self.rotate_r(n_times % self.0.len());
    }

    fn reverse(&mut self, x: usize, y: usize) {
        self.0[x..=y].reverse();
    }

    fn mv(&mut self, x: usize, y: usize) {
        if x < y {
            self.0[x..=y].rotate_left(1);
        } else {
            self.0[y..=x].rotate_right(1);
        }
    }

    fn rotate_reverse(&mut self, c: char) {
        let p = self.position(c);
        let n_times = match p {
            0 => 1,
            1 => 1,
            2 => 6,
            3 => 2,
            4 => 7,
            5 => 3,
            6 => 0,
            7 => 4,
            _ => 0,
        };
        self.rotate_l(n_times % self.0.len());
    }

    fn compute(&mut self, commands: impl IntoIterator<Item = Command>) {
        for cmd in commands {
            self.compute_one(cmd);
        }
    }

    fn compute_one(&mut self, cmd: Command) {
        match cmd {
            Command::Swap(x, y) => self.swap(x, y),
            Command::SwapC(a, b) => self.swap_c(a, b),
            Command::RotateL(steps) => self.rotate_l(steps),
            Command::RotateR(steps) => self.rotate_r(steps),
            Command::RotateC(c) => self.rotate_c(c),
            Command::RotateRev(c) => self.rotate_reverse(c),
            Command::Reverse(x, y) => self.reverse(x, y),
            Command::Move(x, y) => self.mv(x, y),
        }
    }
}

pub fn part_one(input: &str) -> Option<Password> {
    let commands = input.lines().map(|l| l.parse().unwrap());
    let mut pass = Password::from("abcdefgh");
    pass.compute(commands);
    Some(pass)
}

pub fn part_two(input: &str) -> Option<Password> {
    let mut commands = input
        .lines()
        .map(|l| l.parse::<Command>().unwrap().reverse())
        .collect::<Vec<_>>();
    commands.reverse();
    let mut pass = Password::from("fbgdceah");
    pass.compute(commands);
    Some(pass)
}
