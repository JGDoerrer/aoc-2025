advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let coords: Vec<_> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|coord| coord.parse::<u64>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let mut connections: Vec<_> = coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| coords.iter().skip(i).map(|b| (*a, *b)))
        .filter(|(a, b)| a != b)
        .collect();

    connections.sort_by_key(|(a, b)| {
        a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)
    });

    let mut circuits: Vec<Vec<_>> = Vec::new();

    for (a, b) in connections
        .into_iter()
        .take(if coords.len() <= 20 { 10 } else { 1000 })
    {
        match (
            circuits.iter().position(|v| v.contains(&a)),
            circuits.iter().position(|v| v.contains(&b)),
        ) {
            (Some(a), Some(b)) => {
                if a == b {
                    continue;
                }

                let (a, b) = (a.min(b), a.max(b));
                let b = circuits.remove(b);
                circuits[a].extend(b);
            }
            (Some(a), None) => {
                circuits[a].push(b);
            }
            (None, Some(b)) => {
                circuits[b].push(a);
            }
            (None, None) => {
                circuits.push(vec![a, b]);
            }
        }
    }

    let mut lens: Vec<_> = circuits.into_iter().map(|v| v.len() as u64).collect();
    lens.sort();

    Some(lens.into_iter().rev().take(3).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords: Vec<_> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|coord| coord.parse::<u64>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let mut connections: Vec<_> = coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| coords.iter().skip(i).map(|b| (*a, *b)))
        .filter(|(a, b)| a != b)
        .collect();

    connections.sort_by_key(|(a, b)| {
        a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)
    });

    let mut circuits: Vec<Vec<_>> = Vec::new();

    let mut x = 0;

    for (a, b) in connections {
        match (
            circuits.iter().position(|v| v.contains(&a)),
            circuits.iter().position(|v| v.contains(&b)),
        ) {
            (Some(i), Some(j)) => {
                if i == j {
                    continue;
                }

                let (i, j) = (i.min(j), i.max(j));
                let j = circuits.remove(j);
                circuits[i].extend(j);
            }
            (Some(i), None) => {
                circuits[i].push(b);
            }
            (None, Some(j)) => {
                circuits[j].push(a);
            }
            (None, None) => {
                circuits.push(vec![a, b]);
            }
        }
        if circuits.len() == 1 && circuits[0].len() == coords.len() {
            x = a.0 * b.0;
            break;
        }
    }

    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
