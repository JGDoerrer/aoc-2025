advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    #[derive(Debug)]
    enum Op {
        Add,
        Mul,
    }

    let numbers: Vec<Vec<u64>> = input
        .lines()
        .take_while(|line| line.trim_start().starts_with(|c: char| c.is_digit(10)))
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let ops: Vec<_> = input
        .lines()
        .last()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| match c {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => unreachable!(),
                })
                .collect()
        })
        .unwrap();

    let result: u64 = ops
        .into_iter()
        .enumerate()
        .map(|(i, op)| match op {
            Op::Add => numbers.iter().map(|nums| nums[i]).sum::<u64>(),
            Op::Mul => numbers.iter().map(|nums| nums[i]).product(),
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    #[derive(Debug)]
    enum Op {
        Add,
        Mul,
    }

    let lines: Vec<_> = input.lines().collect();

    let ops: Vec<_> = lines
        .last()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| match c {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => unreachable!(),
                })
                .collect()
        })
        .unwrap();

    let mut num_lens: Vec<usize> = lines
        .last()
        .map(|line| {
            line.split(['+', '*'])
                .skip(1)
                .map(|space| space.len())
                .collect()
        })
        .unwrap();
    *num_lens.last_mut().unwrap() += 1;

    let mut all_nums = Vec::new();

    let mut start_index = 0;
    for len in num_lens {
        let mut nums = vec![0u64; len];

        for num in 0..len {
            for i in 0..lines.len() - 1 {
                if lines[i].as_bytes()[start_index + num].is_ascii_whitespace() {
                    continue;
                }

                let digit = lines[i].as_bytes()[start_index + num] - b'0';

                nums[num] *= 10;
                nums[num] += digit as u64;
            }
        }

        all_nums.push(nums);

        start_index += len + 1;
    }

    let result: u64 = ops
        .into_iter()
        .enumerate()
        .map(|(i, op)| match op {
            Op::Add => all_nums[i].iter().sum::<u64>(),
            Op::Mul => all_nums[i].iter().product(),
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
