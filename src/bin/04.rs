use advent_of_code::grid::Grid;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse(input, |c| match c {
        '.' => false,
        '@' => true,
        _ => unreachable!(),
    })
    .unwrap();

    let count = grid
        .find(true)
        .filter(|coord| {
            let neighbours = grid
                .neighbours8(*coord)
                .filter(|c| *grid.get(*c).unwrap())
                .count();

            neighbours < 4
        })
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::parse(input, |c| match c {
        '.' => false,
        '@' => true,
        _ => unreachable!(),
    })
    .unwrap();

    let mut removed = 0;
    loop {
        let accessible = grid.find(true).filter(|coord| {
            let neighbours = grid
                .neighbours8(*coord)
                .filter(|c| *grid.get(*c).unwrap())
                .count();

            neighbours < 4
        });

        let mut new_grid = grid.clone();
        let mut any_removed = false;

        for coord in accessible {
            new_grid.set(coord, false);
            any_removed = true;
            removed += 1;
        }

        grid = new_grid;
        if !any_removed {
            break;
        }
    }

    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
