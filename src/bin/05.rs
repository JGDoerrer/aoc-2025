use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<u64>> = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();
    let ids: Vec<u64> = ids.lines().map(|line| line.parse().unwrap()).collect();

    let mut count = 0;
    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<RangeInclusive<u64>> = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let mut intervals: Vec<RangeInclusive<u64>> = Vec::new();

    while let Some(range) = ranges.pop() {
        let overlap = intervals.iter().position(|r| {
            (r.end() >= range.end() && r.start() <= range.end())
                || (range.end() >= r.end() && range.start() <= r.end())
        });

        if let Some(overlap_index) = overlap {
            let overlap = &intervals[overlap_index];

            let new = *range.start().min(overlap.start())..=*range.end().max(overlap.end());
            intervals.remove(overlap_index);
            ranges.push(new);
        } else {
            intervals.push(range);
        }
    }

    let total = intervals
        .into_iter()
        .map(|range| range.count() as u64)
        .sum();

    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
