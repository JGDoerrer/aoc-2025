advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let turn: u64 = if line.bytes().nth(0).unwrap() == 'L' as u8 {
                    100 - ((line[1..].parse::<u64>().unwrap()) % 100)
                } else {
                    line[1..].parse().unwrap()
                };
                turn
            })
            .fold((50, 0), |(state, count), turn| {
                (
                    (state + turn) % 100,
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
                let turn: u64 = line[1..].parse().unwrap();
                let left = line.bytes().nth(0).unwrap() == 'L' as u8;
                (left, turn)
            })
            .fold((50, 0), |(mut state, mut count), (left, turn)| {
                if left {
                    for _ in 0..turn {
                        state = (state + 99) % 100;
                        if state == 0 {
                            count += 1;
                        }
                    }
                } else {
                    for _ in 0..turn {
                        state = (state + 1) % 100;
                        if state == 0 {
                            count += 1;
                        }
                    }
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
        assert_eq!(result, None);
    }
}
