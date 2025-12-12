use std::{mem, slice::SliceIndex};

advent_of_code::solution!(10);

pub struct TupleIteratorDyn {
    n: usize,
    k: usize,
    current: Option<Box<[usize]>>,
}

impl TupleIteratorDyn {
    pub fn new(n: usize, k: usize) -> Self {
        assert!(n >= k);
        let mut first = vec![0; k].into_boxed_slice();

        for i in 0..k {
            first[i] = i;
        }

        TupleIteratorDyn {
            n,
            k,
            current: Some(first),
        }
    }
}

impl Iterator for TupleIteratorDyn {
    type Item = Box<[usize]>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.as_mut()?;

        let prev = current.clone();

        if current.first().is_some_and(|v| *v == self.n - self.k) {
            self.current = None;
        } else {
            for i in (0..self.k).rev() {
                if current[i] < self.n + i - self.k {
                    current[i] += 1;
                    for j in i + 1..self.k {
                        current[j] = current[i] + j - i;
                    }

                    break;
                }
            }
        }

        Some(prev)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Vec<_> = input
        .lines()
        .map(|line| {
            let split = line.split_once(']').unwrap();
            let target: Vec<_> = split.0[1..]
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect();
            let rest: Vec<_> = split.1[1..].split(' ').collect();
            let buttons: Vec<Vec<_>> = rest[..rest.len() - 1]
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();

            (target, buttons)
        })
        .collect();

    let mut total = 0;

    'input: for (target, buttons) in input {
        for i in 1..=buttons.len() {
            for pressed_buttons in TupleIteratorDyn::new(buttons.len(), i) {
                let mut state = vec![false; target.len()];

                for button in pressed_buttons {
                    let button = &buttons[button];

                    for index in button {
                        state[*index] ^= true;
                    }
                }

                if state == target {
                    total += i as u64;
                    continue 'input;
                }
            }
        }
    }

    Some(total)
}

#[derive(Debug)]
pub struct PartitionIter {
    sum: usize,
    n: usize,
    prev: Option<Box<PartitionIter>>,
    i: usize,
}

impl PartitionIter {
    pub fn new(sum: usize, n: usize) -> Self {
        let prev = if n == 0 {
            None
        } else {
            Some(Box::new(PartitionIter::new(sum, n - 1)))
        };

        Self { sum, n, prev, i: 0 }
    }
}

impl Iterator for PartitionIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            let done = self.i == 1;
            self.i = 1;
            return if done { None } else { Some(Vec::new()) };
        }
        if self.n == 1 {
            let done = self.i == 1;
            self.i = 1;
            return if done { None } else { Some(vec![self.sum]) };
        }

        if let Some(prev) = &mut self.prev {
            if let Some(mut next) = prev.next() {
                next.insert(0, self.i);

                return Some(next);
            } else {
                self.i += 1;
                if self.i <= self.sum {
                    self.prev = Some(Box::new(PartitionIter::new(self.sum - self.i, self.n - 1)));
                } else {
                    self.prev = None;
                };
                return self.next();
            }
        } else {
            return None;
        }
    }
}

fn partitions(sum: usize, n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![vec![sum]];
    }

    let mut result = Vec::new();

    for i in 0..=sum {
        let p = partitions(sum - i, n - 1);
        result.extend(p.into_iter().map(|mut p| {
            p.insert(0, i);
            p
        }));
    }

    result
}

fn solve_rec(buttons: Vec<Vec<usize>>, target: Vec<usize>) -> Option<u64> {
    if target.is_empty() {
        return Some(0);
    }

    let affecting_buttons: Vec<_> = buttons
        .iter()
        .enumerate()
        .filter_map(|(j, b)| {
            if b.contains(&(target.len() - 1)) {
                Some(j)
            } else {
                None
            }
        })
        .collect();

    let sum = *target.last().unwrap();

    let mut min_presses = u64::MAX;

    // dbg!(
    //     partitions(sum, affecting_buttons.len()),
    //     PartitionIter::new(sum, affecting_buttons.len()).collect::<Vec<_>>()
    // );
    // todo!();

    'partitions: for p in PartitionIter::new(sum, affecting_buttons.len()) {
        let mut target_left = target.clone();

        for (i, presses) in p.into_iter().enumerate() {
            for b in &buttons[affecting_buttons[i]] {
                if target_left[*b] < presses {
                    continue 'partitions;
                }

                target_left[*b] -= presses;
            }
        }

        let new_buttons = buttons
            .iter()
            .map(|b| {
                let mut b = b.clone();
                b.retain(|n| *n != target.len() - 1);
                b
            })
            .filter(|b| !b.is_empty())
            .collect();
        debug_assert_eq!(target_left.last(), Some(&0));
        target_left.pop();

        if let Some(presses) = solve_rec(new_buttons, target_left) {
            min_presses = min_presses.min(sum as u64 + presses);
        }
    }

    if min_presses == u64::MAX {
        None
    } else {
        Some(min_presses)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Vec<_> = input
        .lines()
        .map(|line| {
            let split = line.split_once(']').unwrap();
            let rest: Vec<_> = split.1[1..].split(' ').collect();
            let buttons: Vec<Vec<_>> = rest[..rest.len() - 1]
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            let counters = rest.last().unwrap();
            let counters: Vec<usize> = counters[1..counters.len() - 1]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            (buttons, counters)
        })
        .collect();

    let mut total = 0;

    for (buttons, counters) in input {
        total += dbg!(solve_rec(buttons, counters).unwrap());
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
