use md5::{Digest, Md5};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<String> {
    let mut i = 0;
    let mut out = String::with_capacity(8);
    let input = input.trim();
    let binput = input.as_bytes();
    while out.len() < 8 {
        let mut hasher = Md5::new();
        hasher.update(binput);
        let num_str = i.to_string();
        hasher.update(num_str.as_bytes());
        let done = hasher.finalize();
        if done[0] == 0 && done[1] == 0 && done[2] < 16 {
            out.push_str(&format!("{:x}", done[2] & 0b00001111))
        }
        i += 1;
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut i = 0;
    let mut out = ['X'; 8];
    let mut ctr = 0;
    let input = input.trim();
    let binput = input.as_bytes();
    while ctr < 8 {
        let mut hasher = Md5::new();
        hasher.update(binput);
        let num_str = i.to_string();
        hasher.update(num_str.as_bytes());
        let done = hasher.finalize();
        if done[0] == 0 && done[1] == 0 && done[2] < 16 {
            let i = done[2] & 0b00001111;
            if i < 8 {
                let pos = &mut out[i as usize];
                if pos == &'X' {
                    match (done[3] & 0b11110000) >> 4 {
                        0b0000 => *pos = '0',
                        0b0001 => *pos = '1',
                        0b0010 => *pos = '2',
                        0b0011 => *pos = '3',
                        0b0100 => *pos = '4',
                        0b0101 => *pos = '5',
                        0b0110 => *pos = '6',
                        0b0111 => *pos = '7',
                        0b1000 => *pos = '8',
                        0b1001 => *pos = '9',
                        0b1010 => *pos = 'a',
                        0b1011 => *pos = 'b',
                        0b1100 => *pos = 'c',
                        0b1101 => *pos = 'd',
                        0b1110 => *pos = 'e',
                        0b1111 => *pos = 'f',
                        _ => {}
                    }
                    ctr += 1;
                }
            }
        }
        i += 1;
    }
    Some(out.iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("18f47a30".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("05ace8e3".into()));
    }
}
