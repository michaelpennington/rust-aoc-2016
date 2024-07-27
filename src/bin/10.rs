use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Destination {
    Bot(u32),
    Output(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Bot {
    low: Destination,
    high: Destination,
    contents: Contents,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Contents {
    Empty,
    One(u32),
    Two(u32, u32),
}

impl Contents {
    fn insert(&mut self, val: u32) {
        match self {
            Contents::Empty => *self = Contents::One(val),
            Contents::One(a) => *self = Contents::Two(*a, val),
            Contents::Two(_, _) => panic!("Tried to insert val into bot with two contents"),
        }
    }

    fn has_two(&self) -> bool {
        matches!(self, Contents::Two(_, _))
    }
}

#[derive(Clone, Debug)]
struct Map {
    bots: HashMap<u32, Bot>,
    outputs: HashMap<u32, Vec<u32>>,
}

impl Map {
    /// Returns Ok(u32) if we found the bot that compares 61 and 17
    /// Returns Err(true) if we are done checking for progress
    /// Returns Err(false) if there are additional pairs to be processed
    /// Invariant: There are never two bots with two contents
    fn check_if_done(&mut self) -> Result<u32, bool> {
        let mut process = None;
        for (bot_id, bot) in &mut self.bots {
            if let Contents::Two(a, b) = bot.contents {
                if (a == 61 && b == 17) || (a == 17 && b == 61) {
                    return Ok(*bot_id);
                } else {
                    process = Some(((a, b), bot.low, bot.high));
                    bot.contents = Contents::Empty;
                    break;
                }
            }
        }
        if let Some(((a, b), low, high)) = process {
            match (a.cmp(&b), low, high) {
                (
                    Ordering::Less | Ordering::Equal,
                    Destination::Bot(a_dest),
                    Destination::Bot(b_dest),
                ) => {
                    self.bots.get_mut(&a_dest).unwrap().contents.insert(a);
                    self.bots.get_mut(&b_dest).unwrap().contents.insert(b);
                }
                (
                    Ordering::Less | Ordering::Equal,
                    Destination::Bot(a_dest),
                    Destination::Output(b_dest),
                ) => {
                    self.bots.get_mut(&a_dest).unwrap().contents.insert(a);
                    self.outputs.get_mut(&b_dest).unwrap().push(b);
                }
                (
                    Ordering::Less | Ordering::Equal,
                    Destination::Output(a_dest),
                    Destination::Bot(b_dest),
                ) => {
                    self.outputs.get_mut(&a_dest).unwrap().push(a);
                    self.bots.get_mut(&b_dest).unwrap().contents.insert(b);
                }
                (
                    Ordering::Less | Ordering::Equal,
                    Destination::Output(a_dest),
                    Destination::Output(b_dest),
                ) => {
                    self.outputs.get_mut(&a_dest).unwrap().push(a);
                    self.outputs.get_mut(&b_dest).unwrap().push(b);
                }
                (Ordering::Greater, Destination::Bot(a_dest), Destination::Bot(b_dest)) => {
                    self.bots.get_mut(&a_dest).unwrap().contents.insert(b);
                    self.bots.get_mut(&b_dest).unwrap().contents.insert(a);
                }
                (Ordering::Greater, Destination::Bot(a_dest), Destination::Output(b_dest)) => {
                    self.bots.get_mut(&a_dest).unwrap().contents.insert(b);
                    self.outputs.get_mut(&b_dest).unwrap().push(a);
                }
                (Ordering::Greater, Destination::Output(a_dest), Destination::Bot(b_dest)) => {
                    self.outputs.get_mut(&a_dest).unwrap().push(b);
                    self.bots.get_mut(&b_dest).unwrap().contents.insert(a);
                }
                (Ordering::Greater, Destination::Output(a_dest), Destination::Output(b_dest)) => {
                    self.outputs.get_mut(&a_dest).unwrap().push(b);
                    self.outputs.get_mut(&b_dest).unwrap().push(a);
                }
            }
            if self.bots.values().any(|val| val.contents.has_two()) {
                Err(false)
            } else {
                Err(true)
            }
        } else {
            Err(true)
        }
    }

    fn check_if_donev2(&mut self) -> Result<(), ()> {
        let mut process = None;
        for bot in self.bots.values_mut() {
            if let Contents::Two(a, b) = bot.contents {
                process = Some(((a, b), bot.low, bot.high));
                bot.contents = Contents::Empty;
                break;
            }
        }
        if let Some(((a, b), low, high)) = process {
            match (a.cmp(&b), low, high) {
                (
                    Ordering::Less | Ordering::Equal,
                    Destination::Bot(a_dest),
                    Destination::Bot(b_dest),
                ) => {
                    self.bots.get_mut(&a_dest).unwrap().contents.insert(a);
                    self.bots.get_mut(&b_dest).unwrap().contents.insert(b);
                }
                (
                    Ordering::Less | Ordering::Equal,
                    Destination::Bot(a_dest),
                    Destination::Output(b_dest),
                ) => {
                    self.bots.get_mut(&a_dest).unwrap().contents.insert(a);
                    self.outputs.get_mut(&b_dest).unwrap().push(b);
                }
                (
                    Ordering::Less | Ordering::Equal,
                    Destination::Output(a_dest),
                    Destination::Bot(b_dest),
                ) => {
                    self.outputs.get_mut(&a_dest).unwrap().push(a);
                    self.bots.get_mut(&b_dest).unwrap().contents.insert(b);
                }
                (
                    Ordering::Less | Ordering::Equal,
                    Destination::Output(a_dest),
                    Destination::Output(b_dest),
                ) => {
                    self.outputs.get_mut(&a_dest).unwrap().push(a);
                    self.outputs.get_mut(&b_dest).unwrap().push(b);
                }
                (Ordering::Greater, Destination::Bot(a_dest), Destination::Bot(b_dest)) => {
                    self.bots.get_mut(&a_dest).unwrap().contents.insert(b);
                    self.bots.get_mut(&b_dest).unwrap().contents.insert(a);
                }
                (Ordering::Greater, Destination::Bot(a_dest), Destination::Output(b_dest)) => {
                    self.bots.get_mut(&a_dest).unwrap().contents.insert(b);
                    self.outputs.get_mut(&b_dest).unwrap().push(a);
                }
                (Ordering::Greater, Destination::Output(a_dest), Destination::Bot(b_dest)) => {
                    self.outputs.get_mut(&a_dest).unwrap().push(b);
                    self.bots.get_mut(&b_dest).unwrap().contents.insert(a);
                }
                (Ordering::Greater, Destination::Output(a_dest), Destination::Output(b_dest)) => {
                    self.outputs.get_mut(&a_dest).unwrap().push(b);
                    self.outputs.get_mut(&b_dest).unwrap().push(a);
                }
            }
            if self.bots.values().any(|val| val.contents.has_two()) {
                Err(())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn process(&mut self, starting_values: HashMap<u32, u32>) -> Option<u32> {
        for (value, bot) in starting_values {
            self.bots.get_mut(&bot).unwrap().contents.insert(value);
        }
        loop {
            match self.check_if_done() {
                Ok(a) => return Some(a),
                Err(true) => break,
                _ => {}
            }
        }
        None
    }

    fn processv2(&mut self, starting_values: HashMap<u32, u32>) -> Option<u32> {
        for (value, bot) in starting_values {
            self.bots.get_mut(&bot).unwrap().contents.insert(value);
        }
        while self.check_if_donev2().is_err() {}
        Some(
            self.outputs[&0].first().unwrap()
                * self.outputs[&1].first().unwrap()
                * self.outputs[&2].first().unwrap(),
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();
    let mut starting_values = HashMap::new();
    for line in input.lines() {
        if line.contains("gives") {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let (dest_a, dest_b) = match (words[5], words[10]) {
                ("bot", "bot") => (
                    Destination::Bot(words[6].parse().unwrap()),
                    Destination::Bot(words[11].parse().unwrap()),
                ),
                ("bot", "output") => {
                    let output_num = words[11].parse().unwrap();
                    outputs.insert(output_num, Vec::new());
                    (
                        Destination::Bot(words[6].parse().unwrap()),
                        Destination::Output(output_num),
                    )
                }
                ("output", "bot") => {
                    let output_num = words[6].parse().unwrap();
                    outputs.insert(output_num, Vec::new());
                    (
                        Destination::Output(output_num),
                        Destination::Bot(words[11].parse().unwrap()),
                    )
                }
                ("output", "output") => {
                    let output_num1 = words[6].parse().unwrap();
                    let output_num2 = words[11].parse().unwrap();
                    outputs.insert(output_num1, Vec::new());
                    outputs.insert(output_num2, Vec::new());
                    (
                        Destination::Output(output_num1),
                        Destination::Output(output_num2),
                    )
                }
                _ => panic!("Invalid string {}", line),
            };
            bots.insert(
                words[1].parse().unwrap(),
                Bot {
                    low: dest_a,
                    high: dest_b,
                    contents: Contents::Empty,
                },
            );
        } else {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let value = words[1].parse().unwrap();
            let bot_no = words[5].parse().unwrap();
            starting_values.insert(value, bot_no);
        }
    }
    let mut map = Map { bots, outputs };
    map.process(starting_values)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();
    let mut starting_values = HashMap::new();
    for line in input.lines() {
        if line.contains("gives") {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let (dest_a, dest_b) = match (words[5], words[10]) {
                ("bot", "bot") => (
                    Destination::Bot(words[6].parse().unwrap()),
                    Destination::Bot(words[11].parse().unwrap()),
                ),
                ("bot", "output") => {
                    let output_num = words[11].parse().unwrap();
                    outputs.insert(output_num, Vec::new());
                    (
                        Destination::Bot(words[6].parse().unwrap()),
                        Destination::Output(output_num),
                    )
                }
                ("output", "bot") => {
                    let output_num = words[6].parse().unwrap();
                    outputs.insert(output_num, Vec::new());
                    (
                        Destination::Output(output_num),
                        Destination::Bot(words[11].parse().unwrap()),
                    )
                }
                ("output", "output") => {
                    let output_num1 = words[6].parse().unwrap();
                    let output_num2 = words[11].parse().unwrap();
                    outputs.insert(output_num1, Vec::new());
                    outputs.insert(output_num2, Vec::new());
                    (
                        Destination::Output(output_num1),
                        Destination::Output(output_num2),
                    )
                }
                _ => panic!("Invalid string {}", line),
            };
            bots.insert(
                words[1].parse().unwrap(),
                Bot {
                    low: dest_a,
                    high: dest_b,
                    contents: Contents::Empty,
                },
            );
        } else {
            let words = line.split_whitespace().collect::<Vec<_>>();
            let value = words[1].parse().unwrap();
            let bot_no = words[5].parse().unwrap();
            starting_values.insert(value, bot_no);
        }
    }
    let mut map = Map { bots, outputs };
    map.processv2(starting_values)
}
