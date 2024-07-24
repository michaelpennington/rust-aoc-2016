use std::str::FromStr;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Rect { x: usize, y: usize },
    Rotate { axis: Axis, coord: usize, by: usize },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "rect" => {
                let (x, y) = parts[1]
                    .split_once('x')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .ok_or(())?;
                Ok(Self::Rect { x, y })
            }
            "rotate" => {
                let axis = match parts[1] {
                    "row" => Axis::Row,
                    "column" => Axis::Column,
                    _ => return Err(()),
                };
                let coord = parts[2]
                    .split_once('=')
                    .map(|(_, c)| c.parse().unwrap())
                    .ok_or(())?;
                let by = parts[4].parse().map_err(|_| ())?;
                Ok(Self::Rotate { axis, coord, by })
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Display([[bool; 50]; 6]);

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0 {
            for item in line.iter().map(|l| if *l { 'X' } else { '.' }) {
                write!(f, "{item}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display {
    fn rect(&mut self, x: usize, y: usize) {
        self.0[..y]
            .iter_mut()
            .for_each(|xs| xs[..x].iter_mut().for_each(|x| *x = true));
    }

    fn rotate_column(&mut self, x: usize, step: usize) {
        let column: [bool; 6] = self
            .0
            .iter()
            .map(|xs| xs[x])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self.0
            .iter_mut()
            .enumerate()
            .for_each(|(i, xs)| xs[x] = column[(i + 6 - step) % 6])
    }

    fn rotate_row(&mut self, y: usize, step: usize) {
        let row = self.0[y];
        self.0[y]
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = row[(i + 50 - step) % 50])
    }

    fn process(&mut self, instructions: &[Instruction]) {
        // println!("{self}");
        for inst in instructions {
            match inst {
                Instruction::Rect { x, y } => self.rect(*x, *y),
                Instruction::Rotate {
                    axis: Axis::Row,
                    coord,
                    by,
                } => self.rotate_row(*coord, *by),
                Instruction::Rotate {
                    axis: Axis::Column,
                    coord,
                    by,
                } => self.rotate_column(*coord, *by),
            }
            // println!("{self}");
        }
    }

    fn count(&self) -> u32 {
        self.0.iter().flatten().filter(|&p| *p).count() as u32
    }
}

impl Default for Display {
    fn default() -> Self {
        Self([[false; 50]; 6])
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut disp = Display::default();
    disp.process(&instructions);
    Some(disp.count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut disp = Display::default();
    disp.process(&instructions);
    println!("{disp}");
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
