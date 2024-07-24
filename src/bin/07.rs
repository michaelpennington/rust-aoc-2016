use std::collections::HashSet;

advent_of_code::solution!(7);

fn contains_abba(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    for win in chars.windows(4) {
        if win[0] == win[3] && win[0] != win[1] && win[1] == win[2] {
            return true;
        }
    }
    false
}

fn supports_tls(s: &str) -> bool {
    let mut in_brackets = false;
    let mut works = false;
    for part in s.split(['[', ']']) {
        if in_brackets && contains_abba(part) {
            return false;
        }
        if !in_brackets && contains_abba(part) {
            works = true;
        }
        in_brackets = !in_brackets;
    }
    works
}

fn get_abas(s: &str) -> Vec<(char, char)> {
    let mut out = Vec::new();
    let chars = s.chars().collect::<Vec<_>>();
    for win in chars.windows(3) {
        if win[0] == win[2] && win[0] != win[1] {
            out.push((win[0], win[1]));
        }
    }
    out
}

fn supports_ssl(s: &str) -> bool {
    let mut in_brackets = false;
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    for part in s.split(['[', ']']) {
        for aba in get_abas(part) {
            if in_brackets {
                babs.insert((aba.1, aba.0));
            } else {
                abas.insert(aba);
            }
        }
        in_brackets = !in_brackets
    }
    abas.intersection(&babs).count() != 0
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter(|&s| supports_tls(s)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().filter(|&s| supports_ssl(s)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }
}
