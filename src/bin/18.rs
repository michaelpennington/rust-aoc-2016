use std::str::FromStr;

advent_of_code::solution!(18);

const NUM_TILES: usize = 100;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Safe
    }
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Safe),
            '^' => Ok(Self::Trap),
            c => Err(format!("Invalid tile {c}")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Row {
    tiles: [Tile; NUM_TILES],
}

impl Default for Row {
    fn default() -> Self {
        Self {
            tiles: [Default::default(); NUM_TILES],
        }
    }
}

impl FromStr for Row {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = [Tile::Safe; 100];
        for (i, c) in s.chars().enumerate() {
            if let Some(t) = tiles.get_mut(i) {
                *t = c.try_into()?;
            } else {
                return Err(format!("Index {i} is out of bounds"));
            }
        }
        Ok(Self { tiles })
    }
}

impl Row {
    fn next(&self) -> Self {
        let mut new = Self::default();
        for (i, t) in new.tiles.iter_mut().enumerate() {
            *t = self.next_tile(i);
        }
        new
    }

    fn next_tile(&self, index: usize) -> Tile {
        let left = if let Some(lindex) = index.checked_sub(1) {
            self.tiles.get(lindex).copied().unwrap_or_default()
        } else {
            Tile::default()
        };
        let right = self.tiles.get(index + 1).copied().unwrap_or_default();
        match (left, right) {
            (Tile::Safe, Tile::Trap) | (Tile::Trap, Tile::Safe) => Tile::Trap,
            _ => Tile::Safe,
        }
    }

    fn num_safe(&self) -> u32 {
        self.tiles.iter().filter(|&&t| t == Tile::Safe).count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut row = input.trim().parse::<Row>().ok()?;
    let mut total = 0;
    for _ in 0..40 {
        total += row.num_safe();
        row = row.next();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut row = input.trim().parse::<Row>().ok()?;
    let mut total = 0;
    for _ in 0..400000 {
        total += row.num_safe();
        row = row.next();
    }
    Some(total)
}
