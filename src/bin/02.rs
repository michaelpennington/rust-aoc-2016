use std::{ops::AddAssign, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Button(u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl AddAssign<Dir> for Button {
    fn add_assign(&mut self, rhs: Dir) {
        match rhs {
            Dir::U => self.1 = self.1.saturating_sub(1),
            Dir::D => self.1 = 2.min(self.1 + 1),
            Dir::L => self.0 = self.0.saturating_sub(1),
            Dir::R => self.0 = 2.min(self.0 + 1),
        }
    }
}

impl AddAssign<Dir2> for Button {
    fn add_assign(&mut self, rhs: Dir2) {
        let dir = rhs.0;
        let mut old = *self;
        match dir {
            Dir::U => old.1 = self.1.saturating_sub(1),
            Dir::D => old.1 = 4.min(self.1 + 1),
            Dir::L => old.0 = self.0.saturating_sub(1),
            Dir::R => old.0 = 4.min(self.0 + 1),
        }
        if old.0.abs_diff(2) + old.1.abs_diff(2) <= 2 {
            *self = old;
        }
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "D" => Ok(Self::D),
            "L" => Ok(Self::L),
            "R" => Ok(Self::R),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dir2(Dir);

impl From<Dir> for Dir2 {
    fn from(value: Dir) -> Self {
        Self(value)
    }
}

impl Button {
    fn num(&self) -> char {
        match self.0 + (self.1 * 3) + 1 {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => 'X',
        }
    }

    fn num2(&self) -> char {
        match (self.0, self.1) {
            (2, 0) => '1',
            (1, 1) => '2',
            (2, 1) => '3',
            (3, 1) => '4',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => 'A',
            (2, 3) => 'B',
            (3, 3) => 'C',
            (2, 4) => 'D',
            _ => unreachable!(),
        }
    }

    fn calc(&mut self, dirs: impl IntoIterator<Item = Dir>) -> char {
        for dir in dirs {
            *self += dir;
        }
        self.num()
    }

    fn calc2(&mut self, dirs: impl IntoIterator<Item = Dir2>) -> char {
        for dir in dirs {
            *self += dir;
        }
        self.num2()
    }
}

impl Default for Button {
    fn default() -> Self {
        Self(1, 1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut out_num = String::with_capacity(input.lines().count());
    let mut keys = Button::default();
    for dirs in input
        .lines()
        .map(|l| l.split("").filter_map(|s| s.parse::<Dir>().ok()))
    {
        out_num.push(keys.calc(dirs));
    }
    out_num.parse().ok()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut out_num = String::with_capacity(input.lines().count());
    let mut keys = Button(0, 2);
    for dirs in input.lines().map(|l| {
        l.split("")
            .filter_map(|s| s.parse::<Dir>().map(|d| d.into()).ok())
    }) {
        out_num.push(keys.calc2(dirs));
    }
    Some(out_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1985));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5DB3".into()));
    }
}
