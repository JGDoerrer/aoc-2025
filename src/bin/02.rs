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

    let mut total = 0;
    for (min, max) in ranges {
        for num in min..=max {
            let str = num.to_string();

            if str.len() % 2 != 0 {
                continue;
            }
            if str == format!("{}{0}", &str[0..str.len() / 2]) {
                total += num;
            }
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

    let mut total = 0;
    for (min, max) in ranges {
        for num in min..=max {
            let str = num.to_string();

            for i in 1..str.len() {
                let repeat = str[0..i].repeat(str.len() / i);
                if str == repeat {
                    total += num;
                    break;
                }
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
