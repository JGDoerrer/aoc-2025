advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .fold((50, 0), |(state, count), line| {
                let bytes = line.as_bytes();
                let mut num = 0;

                for i in 1..bytes.len() {
                    num = num * 10 + (bytes[i] - '0' as u8) as u64;
                }

                let turn = if bytes[0] == 'L' as u8 {
                    100 - num % 100
                } else {
                    num
                };

                (
                    state + turn,
                    count + if (state + turn) % 100 == 0 { 1 } else { 0 },
                )
            })
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let bytes = line.as_bytes();
                let mut num = 0;

                for i in 1..bytes.len() {
                    num = num * 10 + (bytes[i] - '0' as u8) as u64;
                }

                (bytes[0] == 'L' as u8, num)
            })
            .fold((50, 0), |(mut state, mut count), (left, turn)| {
                if left {
                    if state != 0 && state <= turn % 100 {
                        count += 1;
                    }
                    count += turn / 100;

                    state = (state + (100 - turn % 100)) % 100;
                } else {
                    count += (state + turn) / 100;

                    state = (state + turn) % 100;
                }

                (state, count)
            })
            .1,
    )
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
        assert_eq!(result, Some(6));
    }
}
