advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut max_area = 0;

    for a in coords.iter().cloned() {
        for b in coords.iter().cloned() {
            let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);

            max_area = max_area.max(area);
        }
    }

    Some(max_area)
}

fn get_corner_sg(coords: &[(u64, u64)], i: usize) -> (i64, i64) {
    let coord = coords[i];
    let next = coords[(i + 1) % coords.len()];
    let prev = coords[(i + coords.len() - 1) % coords.len()];

    let diff_x1 = next.0 as i64 - coord.0 as i64;
    let diff_x2 = prev.0 as i64 - coord.0 as i64;
    let diff_y1 = next.1 as i64 - coord.1 as i64;
    let diff_y2 = prev.1 as i64 - coord.1 as i64;

    let sg_x = if diff_x1 == 0 { diff_x2 } else { diff_x1 }.signum();
    let sg_y = if diff_y1 == 0 { diff_y2 } else { diff_y1 }.signum();

    (sg_x, sg_y)
}

fn is_in_loop(
    coords: &[(u64, u64)],
    edges: &[((u64, u64), (u64, u64))],
    coord: (u64, u64),
) -> bool {
    let (x, y) = coord;

    let mut intersections = 0;

    for (a, b) in edges
        .iter()
        .cloned()
        .filter(|(a, b)| a.1.min(b.1) <= y && y <= a.1.max(b.1) && a.0.max(b.0) >= x)
    {
        if a.0 == b.0 {
            if x == a.0 && a.1.min(b.1) <= y && y <= a.1.max(b.1) {
                return true;
            }

            if y != a.1 && y != b.1 {
                intersections += 1;
            }
        } else {
            if y == a.1 && a.0.min(b.0) <= x && x <= a.0.max(b.0) {
                return true;
            }

            let (_sgax, sgay) = get_corner_sg(coords, coords.iter().position(|c| *c == a).unwrap());
            let (_sgbx, sgby) = get_corner_sg(coords, coords.iter().position(|c| *c == b).unwrap());

            if sgay != sgby {
                intersections += 1;
            }
        }
    }

    intersections % 2 == 1
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let edges: Vec<_> = (0..coords.len())
        .map(|i| (coords[i], coords[(i + 1) % coords.len()]))
        .collect();

    let mut max_area = 0;

    // for y in 0..=15 {
    //     for x in 0..=15 {
    //         if coords.contains(&(x, y)) {
    //             print!("X")
    //         } else if is_in_loop(&coords, &edges, (x, y)) {
    //             print!("#")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!()
    // }

    for (i, a) in coords.iter().cloned().enumerate() {
        'rect: for (_j, b) in coords.iter().cloned().enumerate().skip(i + 1) {
            for x in (a.0.min(b.0))..=(a.0.max(b.0)) {
                if !is_in_loop(&coords, &edges, (x, a.1)) {
                    dbg!(a, b, (x, a.1));
                    continue 'rect;
                }
                if !is_in_loop(&coords, &edges, (x, b.1)) {
                    dbg!(a, b, (x, b.1));
                    continue 'rect;
                }
            }
            for y in (a.1.min(b.1))..=(a.1.max(b.1)) {
                if !is_in_loop(&coords, &edges, (a.0, y)) {
                    dbg!(a, b, (a.0, y));
                    continue 'rect;
                }
                if !is_in_loop(&coords, &edges, (b.0, y)) {
                    dbg!(a, b, (b.0, y));
                    continue 'rect;
                }
            }

            let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            dbg!(a, b, area);
            max_area = max_area.max(area);
        }
    }

    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
