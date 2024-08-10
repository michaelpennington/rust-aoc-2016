use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::anyhow;

advent_of_code::solution!(23);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "A" => Ok(Self::A),
            "b" | "B" => Ok(Self::B),
            "c" | "C" => Ok(Self::C),
            "d" | "D" => Ok(Self::D),
            _ => Err(anyhow!("{s} is not a valid register")),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Register::A => 'a',
                Register::B => 'b',
                Register::C => 'c',
                Register::D => 'd',
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    Reg(Register),
    Immediate(isize),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(reg) = s.parse() {
            Ok(Self::Reg(reg))
        } else if let Ok(imm) = s.parse() {
            Ok(Self::Immediate(imm))
        } else {
            Err(anyhow!("{s} must be a register or an immediate value"))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Reg(r) => write!(f, "{r}"),
            Value::Immediate(i) => write!(f, "{i}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Register),
    Dec(Register),
    Jnz(Value, Value),
    Tgl(Register),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next().ok_or(anyhow!("Empty line"))? {
            "cpy" => {
                let v1 = parts
                    .next()
                    .ok_or(anyhow!("cpy takes 2 arguments"))?
                    .parse()?;
                let v2 = parts
                    .next()
                    .ok_or(anyhow!("cpy takes 2 arguments"))?
                    .parse()?;
                Ok(Self::Cpy(v1, v2))
            }
            "inc" => {
                let reg = parts
                    .next()
                    .ok_or(anyhow!("inc takes 1 argument"))?
                    .parse()?;
                Ok(Self::Inc(reg))
            }
            "dec" => {
                let reg = parts
                    .next()
                    .ok_or(anyhow!("dec takes 1 argument"))?
                    .parse()?;
                Ok(Self::Dec(reg))
            }
            "jnz" => {
                let v1 = parts
                    .next()
                    .ok_or(anyhow!("jnz takes 2 arguments"))?
                    .parse()?;
                let v2 = parts
                    .next()
                    .ok_or(anyhow!("jnz takes 2 arguments"))?
                    .parse()?;
                Ok(Self::Jnz(v1, v2))
            }
            "tgl" => {
                let reg = parts
                    .next()
                    .ok_or(anyhow!("tgl takes 1 argument"))?
                    .parse()?;
                Ok(Self::Tgl(reg))
            }
            s => Err(anyhow!("{s} is not a valid instruction")),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Cpy(v1, v2) => write!(f, "cpy {v1} {v2}"),
            Instruction::Inc(r) => write!(f, "inc {r}"),
            Instruction::Dec(r) => write!(f, "dec {r}"),
            Instruction::Jnz(v1, v2) => write!(f, "jnz {v1} {v2}"),
            Instruction::Tgl(r) => write!(f, "tgl {r}"),
        }
    }
}

impl Instruction {
    fn toggle(&self) -> Self {
        match *self {
            Instruction::Cpy(v1, v2) => Self::Jnz(v1, v2),
            Instruction::Inc(r) => Self::Dec(r),
            Instruction::Dec(r) => Self::Inc(r),
            Instruction::Jnz(v1, v2) => Self::Cpy(v1, v2),
            Instruction::Tgl(r) => Self::Inc(r),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Computer {
    registers: [isize; 4],
    pc: usize,
    program: Vec<Instruction>,
}

impl Index<Register> for Computer {
    type Output = isize;

    fn index(&self, index: Register) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<Register> for Computer {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl Computer {
    fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
    }

    fn run(&mut self) -> isize {
        while let Some(inst) = self.program.get(self.pc) {
            match *inst {
                Instruction::Cpy(v1, v2) => match (v1, v2) {
                    (Value::Reg(r1), Value::Reg(r2)) => self[r2] = self[r1],
                    (Value::Immediate(i), Value::Reg(r)) => self[r] = i,
                    (_, Value::Immediate(_)) => {}
                },
                Instruction::Inc(r) => self[r] += 1,
                Instruction::Dec(r) => self[r] -= 1,
                Instruction::Jnz(v1, v2) => {
                    let val = match v1 {
                        Value::Reg(r) => self[r],
                        Value::Immediate(i) => i,
                    };
                    if val != 0 {
                        let offset = match v2 {
                            Value::Reg(r) => self[r],
                            Value::Immediate(i) => i,
                        };
                        self.pc = self.pc.wrapping_add_signed(offset - 1);
                    }
                }
                Instruction::Tgl(r) => {
                    let index = self.pc.wrapping_add_signed(self[r]);
                    if let Some(i) = self.program.get_mut(index) {
                        *i = i.toggle();
                    }
                }
            }

            self.pc += 1;
        }

        self[Register::A]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut comp = Computer {
        registers: [7, 0, 0, 0],
        pc: 0,
        program: Vec::new(),
    };
    let program = input.lines().map(|l| l.parse().unwrap()).collect();
    comp.load_program(program);
    Some(comp.run() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut comp = Computer {
        registers: [12, 0, 0, 0],
        pc: 0,
        program: Vec::new(),
    };
    let program = input.lines().map(|l| l.parse().unwrap()).collect();
    comp.load_program(program);
    Some(comp.run() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
