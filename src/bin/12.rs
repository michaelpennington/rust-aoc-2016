use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

advent_of_code::solution!(12);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Cpy { from: Register, to: Register },
    Cpyi { num: u32, to: Register },
    Inc(Register),
    Dec(Register),
    Jnz(Register, isize),
    Jnzi(u32, isize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct Computer {
    registers: [u32; 4],
    sp: usize,
}

impl Index<Register> for Computer {
    type Output = u32;

    fn index(&self, index: Register) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<Register> for Computer {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(()),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "cpy" => {
                if let Ok(num) = parts[1].parse::<u32>() {
                    let to = parts[2].parse()?;
                    Ok(Self::Cpyi { num, to })
                } else if let Ok(from) = parts[1].parse() {
                    let to = parts[2].parse()?;
                    Ok(Self::Cpy { from, to })
                } else {
                    Err(())
                }
            }
            "inc" => {
                let reg = parts[1].parse()?;
                Ok(Self::Inc(reg))
            }
            "dec" => {
                let reg = parts[1].parse()?;
                Ok(Self::Dec(reg))
            }
            "jnz" => {
                if let Ok(reg) = parts[1].parse() {
                    let inc = parts[2].parse().map_err(|_| ())?;
                    Ok(Self::Jnz(reg, inc))
                } else if let Ok(i) = parts[1].parse() {
                    let inc = parts[2].parse().map_err(|_| ())?;
                    Ok(Self::Jnzi(i, inc))
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

impl Computer {
    fn run(&mut self, program: &[Instruction]) -> u32 {
        while let Some(instruction) = program.get(self.sp) {
            match *instruction {
                Instruction::Cpy { from, to } => self[to] = self[from],
                Instruction::Cpyi { num, to } => self[to] = num,
                Instruction::Inc(r) => self[r] += 1,
                Instruction::Dec(r) => self[r] -= 1,
                Instruction::Jnz(r, inc) => {
                    if self[r] != 0 {
                        self.sp = self.sp.wrapping_add_signed(inc - 1);
                    }
                }
                Instruction::Jnzi(i, inc) => {
                    if i != 0 {
                        self.sp = self.sp.wrapping_add_signed(inc - 1);
                    }
                }
            }
            self.sp += 1;
        }

        self[Register::A]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let program: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut comp = Computer::default();
    Some(comp.run(&program))
}

pub fn part_two(input: &str) -> Option<u32> {
    let program: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut comp = Computer::default();
    comp[Register::C] = 1;
    Some(comp.run(&program))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}
