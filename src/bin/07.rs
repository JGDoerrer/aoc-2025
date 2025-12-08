use advent_of_code::{counting_set::CountingSet, grid::Grid};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    #[derive(Debug, PartialEq, Eq)]
    enum Cell {
        Start,
        Empty,
        Splitter,
    }

    let grid = Grid::parse(input, |c| match c {
        'S' => Cell::Start,
        '.' => Cell::Empty,
        '^' => Cell::Splitter,
        _ => unreachable!(),
    })
    .unwrap();

    let mut splits = 0;

    let mut beams = vec![grid.find(Cell::Start).next().unwrap().1];

    for row in 1..grid.height() {
        let mut new_beams = Vec::new();

        for beam in beams {
            match grid.get((row, beam)).unwrap() {
                Cell::Empty => {
                    new_beams.push(beam);
                }
                Cell::Splitter => {
                    new_beams.push(beam - 1);
                    new_beams.push(beam + 1);
                    splits += 1;
                }
                _ => unreachable!(),
            }
        }

        new_beams.sort();
        new_beams.dedup();
        beams = new_beams;
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    #[derive(Debug, PartialEq, Eq)]
    enum Cell {
        Start,
        Empty,
        Splitter,
    }

    let grid = Grid::parse(input, |c| match c {
        'S' => Cell::Start,
        '.' => Cell::Empty,
        '^' => Cell::Splitter,
        _ => unreachable!(),
    })
    .unwrap();

    let mut splits = 1;

    let mut beams = CountingSet::new();
    beams.insert(grid.find(Cell::Start).next().unwrap().1);

    for row in 1..grid.height() {
        let mut new_beams = CountingSet::new();

        for (beam, count) in beams {
            match grid.get((row, beam)).unwrap() {
                Cell::Empty => {
                    new_beams.insert_count(beam, count);
                }
                Cell::Splitter => {
                    new_beams.insert_count(beam - 1, count);
                    new_beams.insert_count(beam + 1, count);
                    splits += count;
                }
                _ => unreachable!(),
            }
        }

        beams = new_beams;
    }

    Some(splits as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
