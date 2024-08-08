advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let n = input.trim().parse::<u32>().unwrap();
    let m = n.ilog2();
    let l = n - 2u32.pow(m);
    Some(2 * l + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let n = input.trim().parse::<u32>().unwrap();
    let k = (n as f64).log(3.0).floor() as u32;
    let np3 = 3u32.pow(k);
    let b = n - np3;
    if n == np3 {
        Some(n)
    } else if n <= 2 * np3 {
        Some(b)
    } else {
        Some(2 * b - np3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
