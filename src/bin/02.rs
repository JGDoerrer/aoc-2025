advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input
        .trim_end()
        .split(',')
        .map(|range| {
            let (min, max) = range.split_once('-').unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect();

    const POWERS: [u64; 10] = [
        1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
    ];

    let mut total = 0;
    for (min, max) in ranges {
        let mut num = min;
        while num <= max {
            let digits = num.ilog10() + 1;
            if digits % 2 != 0 {
                num = POWERS[digits as usize];
                continue;
            }

            let half = POWERS[digits as usize / 2];

            if num / half == num % half {
                total += num;
            }
            num += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input
        .trim_end()
        .split(',')
        .map(|range| {
            let (min, max) = range.split_once('-').unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect();

    const POWERS: [u64; 10] = [
        1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
    ];

    let mut total = 0;
    for (min, max) in ranges {
        for num in min..=max {
            let digits = num.ilog10() + 1;

            'i: for i in 1..digits {
                if digits % i != 0 {
                    continue;
                }

                let pow = POWERS[i as usize];
                let last_digits = num % pow;
                let mut rest = num;

                for _ in 0..digits / i {
                    if rest % pow != last_digits {
                        continue 'i;
                    }

                    rest /= pow;
                }

                total += num;
                break;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
