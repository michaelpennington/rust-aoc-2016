use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::anyhow;

advent_of_code::solution!(25);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(anyhow!("{s} is not a valid register")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Reg(Register),
    Imm(isize),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(r) = s.parse() {
            Ok(Self::Reg(r))
        } else if let Ok(i) = s.parse() {
            Ok(Self::Imm(i))
        } else {
            Err(anyhow!("{s} is not a valid value"))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Cpy(Value, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Value, Value),
    Out(Register),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let not_enough_data = || anyhow!("{s} does not have enough fields");
        match parts.next().ok_or(not_enough_data())? {
            "cpy" => {
                let val = parts.next().ok_or(not_enough_data())?.parse()?;
                let reg = parts.next().ok_or(not_enough_data())?.parse()?;
                Ok(Self::Cpy(val, reg))
            }
            "inc" => {
                let reg = parts.next().ok_or(not_enough_data())?.parse()?;
                Ok(Self::Inc(reg))
            }
            "dec" => {
                let reg = parts.next().ok_or(not_enough_data())?.parse()?;
                Ok(Self::Dec(reg))
            }
            "jnz" => {
                let v1 = parts.next().ok_or(not_enough_data())?.parse()?;
                let v2 = parts.next().ok_or(not_enough_data())?.parse()?;
                Ok(Self::Jnz(v1, v2))
            }
            "out" => {
                let reg = parts.next().ok_or(not_enough_data())?.parse()?;
                Ok(Self::Out(reg))
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    code: Vec<Instruction>,
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

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code = Vec::with_capacity(s.lines().count());
        for line in s.lines() {
            code.push(line.parse()?);
        }

        Ok(Self { code })
    }
}

impl Program {
    fn get(&self, index: usize) -> Option<&Instruction> {
        self.code.get(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Computer {
    registers: [isize; 4],
    pc: usize,
}

impl Computer {
    fn run(&mut self, program: &Program, num_steps: usize) -> bool {
        let mut count = 0;
        let mut output = Vec::new();
        while let Some(i) = program.get(self.pc) {
            if count >= num_steps {
                break;
            }
            match *i {
                Instruction::Cpy(v1, r) => {
                    self[r] = match v1 {
                        Value::Reg(r1) => self[r1],
                        Value::Imm(i) => i,
                    };
                }
                Instruction::Inc(r) => self[r] += 1,
                Instruction::Dec(r) => self[r] -= 1,
                Instruction::Jnz(v1, v2) => {
                    let v1 = match v1 {
                        Value::Reg(r) => self[r],
                        Value::Imm(i) => i,
                    };
                    if v1 != 0 {
                        let offset = match v2 {
                            Value::Reg(r) => self[r],
                            Value::Imm(i) => i,
                        };
                        self.pc = self.pc.wrapping_add_signed(offset - 1);
                    }
                }
                Instruction::Out(r) => {
                    output.push(self[r]);
                }
            }
            if !output.is_empty()
                && output.len() % 2 == 0
                && output.chunks(2).any(|c| c[0] != 0 || c[1] != 1)
            {
                return false;
            }
            self.pc += 1;
            count += 1;
        }
        !output.is_empty()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = input.parse::<Program>().unwrap();
    for a in 0..5000000 {
        let mut comp = Computer {
            registers: [a, 0, 0, 0],
            pc: 0,
        };
        if comp.run(&instructions, 100000) {
            return Some(a as u32);
        }
    }
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(42)
}
