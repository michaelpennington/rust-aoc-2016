advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<String> {
    let line_len = input.lines().next().unwrap().len();
    let mut letters = vec![[0u32; 26]; line_len];
    for line in input.lines() {
        for (i, ch) in line.char_indices() {
            let index = (ch as u32 - 'a' as u32) as usize;
            letters[i][index] += 1;
        }
    }
    let mut out = String::with_capacity(line_len);
    for larray in letters {
        let mut max_count = 0;
        let mut max_index = 0;
        for (ind, count) in larray.into_iter().enumerate() {
            if count > max_count {
                max_count = count;
                max_index = ind;
            }
        }
        let ch = char::from_u32('a' as u32 + max_index as u32).unwrap();
        out.push(ch);
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<String> {
    let line_len = input.lines().next().unwrap().len();
    let mut letters = vec![[0u32; 26]; line_len];
    for line in input.lines() {
        for (i, ch) in line.char_indices() {
            let index = (ch as u32 - 'a' as u32) as usize;
            letters[i][index] += 1;
        }
    }
    let mut out = String::with_capacity(line_len);
    for larray in letters {
        let mut min_count = u32::MAX;
        let mut min_index = 0;
        for (ind, count) in larray.into_iter().enumerate().filter(|&(_, c)| c != 0) {
            if count < min_count {
                min_count = count;
                min_index = ind;
            }
        }
        let ch = char::from_u32('a' as u32 + min_index as u32).unwrap();
        out.push(ch);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("easter".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("advent".into()));
    }
}
