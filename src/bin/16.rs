use std::{fmt::Display, ops::Not, str::FromStr};

advent_of_code::solution!(16);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Digit {
    One,
    Zero,
}

impl Not for Digit {
    type Output = Digit;

    fn not(self) -> Self::Output {
        match self {
            Digit::One => Digit::Zero,
            Digit::Zero => Digit::One,
        }
    }
}

impl TryFrom<char> for Digit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            _ => Err("Invalid digit"),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Digit::One => '1',
                Digit::Zero => '0',
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DigitString {
    digits: Vec<Digit>,
}

impl FromStr for DigitString {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());
        for c in s.chars() {
            digits.push(c.try_into()?);
        }
        Ok(Self { digits })
    }
}

impl Display for DigitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.digits {
            write!(f, "{digit}")?;
        }
        Ok(())
    }
}

impl DigitString {
    fn len(&self) -> usize {
        self.digits.len()
    }

    fn truncate(&mut self, len: usize) {
        self.digits.truncate(len);
    }

    fn dragon(&mut self, target: usize) {
        while self.len() < target {
            let mut b = self.clone();
            b.digits.reverse();
            b.flip();
            self.digits.push(Digit::Zero);
            self.digits.extend_from_slice(&b.digits);
        }
        self.truncate(target);
    }

    fn flip(&mut self) {
        self.digits.iter_mut().for_each(|d| *d = !*d)
    }

    fn checksum(&self) -> DigitString {
        let mut new = Vec::with_capacity(self.len() / 2);
        let mut old = self.digits.clone();
        while new.len() % 2 == 0 {
            new = Vec::with_capacity(old.len() / 2);
            for pair in old.chunks(2) {
                if pair[0] == pair[1] {
                    new.push(Digit::One);
                } else {
                    new.push(Digit::Zero);
                }
            }
            old = new.clone();
        }
        Self { digits: new }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut digis: DigitString = input.trim().parse().ok()?;
    digis.dragon(272);
    let chksm = digis.checksum();
    Some(format!("{chksm}"))
}

pub fn part_two(input: &str) -> Option<String> {
    let mut digis: DigitString = input.trim().parse().ok()?;
    digis.dragon(35651584);
    let chksm = digis.checksum();
    Some(format!("{chksm}"))
}
