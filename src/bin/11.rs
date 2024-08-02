use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use bitflags::bitflags;

advent_of_code::solution!(11);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct FloorContents: u32 {
        const HYDROGEN_C = 1 << 0;
        const HYDROGEN_G = 1 << 1;
        const LITHIUM_C = 1 << 2;
        const LITHIUM_G = 1 << 3;
        const THULIUM_C = 1 << 4;
        const THULIUM_G = 1 << 5;
        const PLUTONIUM_C = 1 << 6;
        const PLUTONIUM_G = 1 << 7;
        const STRONTIUM_C = 1 << 8;
        const STRONTIUM_G = 1 << 9;
        const PROMETHIUM_C = 1 << 10;
        const PROMETHIUM_G = 1 << 11;
        const RUTHENIUM_C = 1 << 12;
        const RUTHENIUM_G = 1 << 13;
        const ELERIUM_C = 1 << 14;
        const ELERIUM_G = 1 << 15;
        const DILITHIUM_C = 1 << 16;
        const DILITHIUM_G = 1 << 17;
        const CHIPS = 1 << 0
            | 1 << 2
            | 1 << 4
            | 1 << 6
            | 1 << 8
            | 1 << 10
            | 1 << 12
            | 1 << 14
            | 1 << 16;
        const GENERATORS = 1 << 1
            | 1 << 3
            | 1 << 5
            | 1 << 7
            | 1 << 9
            | 1 << 11
            | 1 << 13
            | 1 << 15
            | 1 << 17;
    }
}

impl FromStr for FloorContents {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Self::empty();
        for s in s
            .split(" a ")
            .skip(1)
            .map(|s| s.trim_end_matches(" and").trim_matches([' ', '.', ',']))
        {
            out |= Self::single_from_str(s)?;
        }
        Ok(out)
    }
}

impl Display for FloorContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in Self::all().iter() {
            if self.contains(bit) {
                write!(f, "X  ")?;
            } else {
                write!(f, ".  ")?;
            }
        }
        Ok(())
    }
}

impl Display for Facility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "E HM HG LM LG TM TG PM PG SM SG pM pG RM RG")?;
        for floor in FLOORS.into_iter().rev() {
            if self.elevator == floor {
                write!(f, "> ")?;
            } else {
                write!(f, "  ")?;
            }
            writeln!(f, "{}", self.floors[floor as usize])?;
        }
        Ok(())
    }
}

impl FloorContents {
    fn single_from_str(s: &str) -> Result<Self, ()> {
        match s {
            "thulium generator" => Ok(Self::THULIUM_G),
            "plutonium generator" => Ok(Self::PLUTONIUM_G),
            "strontium generator" => Ok(Self::STRONTIUM_G),
            "promethium generator" => Ok(Self::PROMETHIUM_G),
            "ruthenium generator" => Ok(Self::RUTHENIUM_G),
            "hydrogen generator" => Ok(Self::HYDROGEN_G),
            "lithium generator" => Ok(Self::LITHIUM_G),
            "thulium-compatible microchip" => Ok(Self::THULIUM_C),
            "plutonium-compatible microchip" => Ok(Self::PLUTONIUM_C),
            "strontium-compatible microchip" => Ok(Self::STRONTIUM_C),
            "ruthenium-compatible microchip" => Ok(Self::RUTHENIUM_C),
            "promethium-compatible microchip" => Ok(Self::PROMETHIUM_C),
            "hydrogen-compatible microchip" => Ok(Self::HYDROGEN_C),
            "lithium-compatible microchip" => Ok(Self::LITHIUM_C),
            _ => Err(()),
        }
    }

    fn chips_iter(&self) -> impl Iterator<Item = FloorContents> {
        self.iter().filter(|f| f.intersects(Self::CHIPS))
    }
    //
    // fn generators_iter(&self) -> impl Iterator<Item = FloorContents> {
    //     self.iter().filter(|f| f.intersects(Self::GENERATORS))
    // }

    const fn corresponds_to(&self) -> FloorContents {
        match *self {
            FloorContents::HYDROGEN_G => FloorContents::HYDROGEN_C,
            FloorContents::LITHIUM_G => FloorContents::LITHIUM_C,
            FloorContents::THULIUM_G => FloorContents::THULIUM_C,
            FloorContents::PLUTONIUM_G => FloorContents::PLUTONIUM_C,
            FloorContents::STRONTIUM_G => FloorContents::STRONTIUM_C,
            FloorContents::PROMETHIUM_G => FloorContents::PROMETHIUM_C,
            FloorContents::RUTHENIUM_G => FloorContents::RUTHENIUM_C,
            FloorContents::DILITHIUM_G => FloorContents::DILITHIUM_C,
            FloorContents::ELERIUM_G => FloorContents::ELERIUM_C,
            FloorContents::HYDROGEN_C => FloorContents::HYDROGEN_G,
            FloorContents::LITHIUM_C => FloorContents::LITHIUM_G,
            FloorContents::THULIUM_C => FloorContents::THULIUM_G,
            FloorContents::PLUTONIUM_C => FloorContents::PLUTONIUM_G,
            FloorContents::STRONTIUM_C => FloorContents::STRONTIUM_G,
            FloorContents::PROMETHIUM_C => FloorContents::PROMETHIUM_G,
            FloorContents::RUTHENIUM_C => FloorContents::RUTHENIUM_G,
            FloorContents::DILITHIUM_C => FloorContents::DILITHIUM_G,
            FloorContents::ELERIUM_C => FloorContents::ELERIUM_G,
            _ => FloorContents::empty(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Floor {
    F1 = 0,
    F2 = 1,
    F3 = 2,
    F4 = 3,
}

const FLOORS: [Floor; 4] = [Floor::F1, Floor::F2, Floor::F3, Floor::F4];

impl Floor {
    fn neighbors(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Floor::F1 => Box::new([Floor::F2].into_iter()),
            Floor::F2 => Box::new([Floor::F1, Floor::F3].into_iter()),
            Floor::F3 => Box::new([Floor::F2, Floor::F4].into_iter()),
            Floor::F4 => Box::new([Floor::F3].into_iter()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Facility {
    floors: [FloorContents; 4],
    elevator: Floor,
}

impl Facility {
    fn has_finished(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }

    fn is_valid(&self) -> bool {
        FLOORS.iter().all(|floor| {
            !self[*floor].intersects(FloorContents::CHIPS)
                || !self[*floor].intersects(FloorContents::GENERATORS)
                || self[*floor]
                    .chips_iter()
                    .all(|chip| self[*floor].contains(chip.corresponds_to()))
        })
    }

    fn next_possible_states(&self) -> Vec<Facility> {
        let mut out = Vec::new();
        // dbg!(&self);
        for floor in self.elevator.neighbors() {
            for content in self[self.elevator].iter() {
                let mut new = *self;
                new[self.elevator] &= !content;
                new[floor] |= content;
                new.elevator = floor;
                out.push(new);
            }

            for content in self[self.elevator].iter() {
                for other in self[self.elevator]
                    .iter()
                    .filter(|c| c.bits() > content.bits())
                {
                    let mut new = *self;
                    let new_bits = content | other;
                    new[self.elevator] &= !new_bits;
                    new[floor] |= new_bits;
                    new.elevator = floor;
                    out.push(new);
                }
            }
        }
        // dbg!(&out);
        out.retain(|f| f.is_valid());
        out
    }
}

impl Index<Floor> for Facility {
    type Output = FloorContents;

    fn index(&self, index: Floor) -> &Self::Output {
        &self.floors[index as usize]
    }
}

impl IndexMut<Floor> for Facility {
    fn index_mut(&mut self, index: Floor) -> &mut Self::Output {
        &mut self.floors[index as usize]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let floors: [FloorContents; 4] = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let facility = Facility {
        floors,
        elevator: Floor::F1,
    };
    let mut next = vec![facility];
    for i in 1.. {
        let mut new_next = Vec::with_capacity(next.len());
        for fac in &next {
            let next = fac.next_possible_states();

            new_next.extend_from_slice(&next);
        }
        new_next.sort();
        new_next.dedup();
        for fac in &new_next {
            if fac.has_finished() {
                return Some(i);
            }
        }
        next = new_next;
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let floors: [FloorContents; 4] = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut facility = Facility {
        floors,
        elevator: Floor::F1,
    };
    facility.floors[0] |= FloorContents::ELERIUM_C
        | FloorContents::ELERIUM_G
        | FloorContents::DILITHIUM_C
        | FloorContents::DILITHIUM_G;
    let mut next = vec![facility];
    for i in 1.. {
        let mut new_next = Vec::with_capacity(next.len());
        for fac in &next {
            let next = fac.next_possible_states();

            new_next.extend_from_slice(&next);
        }
        new_next.sort();
        new_next.dedup();
        for fac in &new_next {
            if fac.has_finished() {
                return Some(i);
            }
        }
        next = new_next;
    }
    None
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
