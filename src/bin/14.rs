advent_of_code::solution!(14);

fn contains_three_in_a_row(s: &str) -> Option<char> {
    for ss in s.as_bytes().windows(3) {
        if ss[0] == ss[1] && ss[1] == ss[2] {
            return Some(ss[0] as char);
        }
    }
    None
}

fn contains_five_in_a_row(s: &str) -> Option<char> {
    for ss in s.as_bytes().windows(5) {
        if ss[0] == ss[1] && ss[1] == ss[2] && ss[2] == ss[3] && ss[3] == ss[4] {
            return Some(ss[0] as char);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let mut count = 63;
    for (i, w) in lines.windows(1001).enumerate().filter(|&(_, w)| {
        if let Some(c) = contains_three_in_a_row(w[0]) {
            w.iter()
                .skip(1)
                .any(|w| Some(c) == contains_five_in_a_row(w))
        } else {
            false
        }
    }) {
        println!("Hit: {} at index {i}", w[0]);
        if count == 0 {
            return Some(i as u32);
        } else {
            count -= 1;
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let mut count = 63;
    for (i, w) in lines.windows(1001).enumerate().filter(|&(_, w)| {
        if let Some(c) = contains_three_in_a_row(w[0]) {
            w.iter()
                .skip(1)
                .any(|w| Some(c) == contains_five_in_a_row(w))
        } else {
            false
        }
    }) {
        println!("Hit: {} at index {i}", w[0]);
        if count == 0 {
            return Some(i as u32);
        } else {
            count -= 1;
        }
    }
    None
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(22728));
//     }
// }
