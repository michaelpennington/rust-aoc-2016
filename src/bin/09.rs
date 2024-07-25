advent_of_code::solution!(9);

fn decompressed_len(s: &mut &str) -> usize {
    let mut total = 0;
    while let Some(first_paren) = s.find('(') {
        let i = first_paren + '('.len_utf8();
        total += first_paren;
        let x = s[i..].find('x').unwrap();
        let len = s[i..i + x].parse::<usize>().unwrap();
        let end = s[i + x..].find(')').unwrap();
        let count = s[i + x + 'x'.len_utf8()..i + x + end]
            .parse::<usize>()
            .unwrap();
        total += count * len;
        *s = &s[i + x + end + ')'.len_utf8() + len..]
    }
    total + s.len()
}

fn decompressed_len2(s: &mut &str) -> usize {
    let mut total = 0;
    while let Some(first_paren) = s.find('(') {
        let i = first_paren + '('.len_utf8();
        total += first_paren;
        let x = s[i..].find('x').unwrap();
        let len = s[i..i + x].parse::<usize>().unwrap();
        let end = s[i + x..].find(')').unwrap();
        let count = s[i + x + 'x'.len_utf8()..i + x + end]
            .parse::<usize>()
            .unwrap();
        if s[i + x + end + ')'.len_utf8()..i + x + end + ')'.len_utf8() + len].contains('(') {
            let new_s = &mut &s[i + x + end + ')'.len_utf8()..i + x + end + ')'.len_utf8() + len];
            let new_len = decompressed_len2(new_s);
            total += count * new_len;
        } else {
            total += count * len;
        }
        *s = &s[i + x + end + ')'.len_utf8() + len..]
    }
    total + s.len()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|mut l| decompressed_len(&mut l))
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    // for mut line in input.lines() {
    //     let l = line.to_string();
    //     println!("{l} is len {}", decompressed_len2(&mut line));
    // }
    Some(
        input
            .lines()
            .map(|mut l| decompressed_len2(&mut l))
            .sum::<usize>() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(57));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6 + 7 + 9 + 11 + 3 + 20 + 241920 + 445));
    }
}
