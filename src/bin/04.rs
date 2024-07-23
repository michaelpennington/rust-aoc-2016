use std::{collections::HashMap, fmt::Display, str::FromStr};

advent_of_code::solution!(4);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Room {
    name: String,
    chars: HashMap<char, u32>,
    id: u32,
    checksum: [char; 5],
}

impl FromStr for Room {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (beginning, checksum) = s.trim_end_matches(']').split_once('[').ok_or(())?;
        let begin_end = beginning.rfind('-').ok_or(())? + 1;
        let id = beginning[begin_end..].parse().map_err(|_| ())?;
        let mut chars = HashMap::with_capacity(beginning.len());
        for ch in beginning[..begin_end].chars().filter(|&c| c != '-') {
            chars.entry(ch).and_modify(|n| *n += 1).or_insert(1);
        }
        let checksum = checksum
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ())?;
        Ok(Self {
            name: s.into(),
            chars,
            id,
            checksum,
        })
    }
}

impl Room {
    fn is_valid(&self) -> bool {
        // let mut chars = self.chars.iter().collect::<Vec<_>>();
        // chars.sort_by_key(|c| c.1);
        let mut new_map = HashMap::with_capacity(self.chars.len());
        for (ch, num) in &self.chars {
            new_map
                .entry(num)
                .and_modify(|v: &mut Vec<_>| v.push(ch))
                .or_insert(vec![ch]);
        }
        let mut chars = new_map.into_iter().collect::<Vec<_>>();
        chars.sort_by_key(|ch| ch.0);
        chars.reverse();
        for ch in &mut chars {
            ch.1.sort();
        }
        chars
            .into_iter()
            .flat_map(|ch| ch.1)
            .zip(self.checksum)
            .all(|(&c1, c2)| c1 == c2)
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = self.name.rfind('-').unwrap();
        for ch in self.name[..end].chars() {
            let out_ch = if ch == '-' {
                ' '
            } else {
                ((((ch as u32 - 'a' as u32) + self.id) % 26 + 'a' as u32) as u8) as char
            };
            write!(f, "{out_ch}")?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<Room>().unwrap())
            .filter(|r| r.is_valid())
            .map(|r| r.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let rooms = input
        .lines()
        .map(|l| l.parse::<Room>().unwrap())
        .filter(|r| r.is_valid());
    for room in rooms {
        if room.to_string().contains("north") {
            return Some(room.id);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1514));
    }
}
