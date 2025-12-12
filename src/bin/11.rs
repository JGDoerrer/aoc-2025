use std::collections::HashMap;

use advent_of_code::pathfind_num_ways;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let connections: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let from = &line[0..3];
            let to: Vec<_> = line[5..line.len()].split(' ').collect();
            (from, to)
        })
        .collect();

    let count = pathfind_num_ways("you", "out", |s| {
        connections
            .get(s)
            .iter()
            .flat_map(|c| c.iter().cloned())
            .collect::<Vec<_>>()
            .into_iter()
    });

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let connections: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let from = &line[0..3];
            let to: Vec<_> = line[5..line.len()].split(' ').collect();
            (from, to)
        })
        .collect();

    let count_fft = pathfind_num_ways("svr", "fft", |s| {
        connections
            .get(s)
            .iter()
            .flat_map(|c| c.iter().cloned())
            .collect::<Vec<_>>()
            .into_iter()
    });
    let count_dac = pathfind_num_ways("svr", "dac", |s| {
        connections
            .get(s)
            .iter()
            .flat_map(|c| c.iter().cloned())
            .collect::<Vec<_>>()
            .into_iter()
    });

    let count_fft_dac = count_fft
        * pathfind_num_ways("fft", "dac", |s| {
            connections
                .get(s)
                .iter()
                .flat_map(|c| c.iter().cloned())
                .collect::<Vec<_>>()
                .into_iter()
        });
    let count_dac_fft = count_dac
        * pathfind_num_ways("dac", "fft", |s| {
            connections
                .get(s)
                .iter()
                .flat_map(|c| c.iter().cloned())
                .collect::<Vec<_>>()
                .into_iter()
        });

    let count_out = count_dac_fft
        * pathfind_num_ways("fft", "out", |s| {
            connections
                .get(s)
                .iter()
                .flat_map(|c| c.iter().cloned())
                .collect::<Vec<_>>()
                .into_iter()
        })
        + count_fft_dac
            * pathfind_num_ways("dac", "out", |s| {
                connections
                    .get(s)
                    .iter()
                    .flat_map(|c| c.iter().cloned())
                    .collect::<Vec<_>>()
                    .into_iter()
            });

    Some(count_out as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            &"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
",
        );
        assert_eq!(result, Some(2));
    }
}
