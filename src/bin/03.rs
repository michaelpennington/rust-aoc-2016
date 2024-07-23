advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let nums: [u32; 3] = l
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                nums
            })
            .filter(|nums| {
                let mut nums = *nums;
                nums.sort();
                nums[0] + nums[1] > nums[2]
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|l| {
            let nums: [u32; 3] = l
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            nums
        })
        .collect::<Vec<_>>();
    Some(
        lines
            .chunks(3)
            .flat_map(|ch| {
                [
                    [ch[0][0], ch[1][0], ch[2][0]],
                    [ch[0][1], ch[1][1], ch[2][1]],
                    [ch[0][2], ch[1][2], ch[2][2]],
                ]
            })
            .filter(|nums| {
                let mut nums = *nums;
                nums.sort();
                nums[0] + nums[1] > nums[2]
            })
            .count() as u32,
    )
}
