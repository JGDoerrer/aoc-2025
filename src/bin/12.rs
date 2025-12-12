use advent_of_code::grid::Grid;

advent_of_code::solution!(12);

fn solve(shapes: &[Grid<bool>], grid: Grid<bool>, counts: Vec<usize>) -> bool {
    let Some(i) = counts.iter().position(|c| *c != 0) else {
        return true;
    };

    let sizes: usize = shapes
        .iter()
        .zip(counts.iter())
        .map(|(s, c)| s.find(true).count() * c)
        .sum();
    if sizes > grid.area() {
        return false;
    }

    for rot in 0..4 {
        let mut shape = shapes[i].clone();

        for _ in 0..rot {
            shape = shape.rotated_clockwise();
        }

        for (row, col) in grid.positions() {
            if !can_place(&shape, (row, col), &grid) {
                continue;
            }

            let mut new_grid = grid.clone();
            for ((r, c), _) in shape.position_values().filter(|(_, b)| **b) {
                new_grid.set((row + r, col + c), true);
            }

            let mut new_counts = counts.clone();
            new_counts[i] -= 1;

            if solve(shapes, new_grid, new_counts) {
                return true;
            }
        }
    }

    false
}

fn can_place(shape: &Grid<bool>, pos: (usize, usize), grid: &Grid<bool>) -> bool {
    for ((row, col), _) in shape.position_values().filter(|(_, b)| **b) {
        if grid.get((row + pos.0, col + pos.1)).is_none_or(|b| *b) {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let num_shapes = 6;
    let size = 3;

    let mut lines = input.lines();

    let shapes: Vec<_> = (0..num_shapes)
        .map(|_| {
            lines.next();

            let shape: Vec<_> = (0..size)
                .map(|_| {
                    let row: Vec<_> = lines
                        .next()
                        .unwrap()
                        .trim_end()
                        .chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => unreachable!(),
                        })
                        .collect();
                    row
                })
                .collect();
            lines.next();
            Grid::from_vec_rows(&shape).unwrap()
        })
        .collect();

    let puzzles: Vec<_> = lines
        .map(|line| {
            let (size, counts) = line.split_once(':').unwrap();
            let (x, y) = size.split_once('x').unwrap();
            let size: (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());

            let counts: Vec<usize> = counts
                .trim_start()
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect();

            (size, counts)
        })
        .collect();

    let mut possible = 0;

    for ((w, h), counts) in puzzles {
        let area = Grid::new(w, h);
        if solve(&shapes, area, counts) {
            possible += 1;
        }
    }

    Some(possible)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
