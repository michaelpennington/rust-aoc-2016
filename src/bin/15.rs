advent_of_code::solution!(15);

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(prod - (sum % prod))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut residues = Vec::new();
    let mut modulii = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let p = parts[3].parse().unwrap();
        let r: i64 = parts[11].trim_end_matches('.').parse().unwrap();
        let r = (r + i as i64 + 1) % p;
        residues.push(r);
        modulii.push(p);
    }
    chinese_remainder(&residues, &modulii).map(|n| n as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut residues = Vec::new();
    let mut modulii = Vec::new();
    let mut last_i = 0;
    for (i, line) in input.lines().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let p = parts[3].parse().unwrap();
        let r: i64 = parts[11].trim_end_matches('.').parse().unwrap();
        let r = (r + i as i64 + 1) % p;
        residues.push(r);
        modulii.push(p);
        last_i = i;
    }
    residues.push(last_i as i64 + 2);
    modulii.push(11);
    chinese_remainder(&residues, &modulii).map(|n| n as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
