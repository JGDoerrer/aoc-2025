use std::{cmp::Reverse, usize};

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
            return if done {
                None
            } else {
                Some(Vec::with_capacity(16))
            };
        }
        if self.n == 1 {
            let done = self.i == 1;
            self.i = 1;

            return if done {
                None
            } else {
                let mut vec = Vec::with_capacity(16);
                vec.push(self.sum);
                Some(vec)
            };
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

fn next_partition(sum: usize, partition: &mut [usize], state: &mut [usize]) -> bool {
    if state.is_empty() {
        return true;
    }
    if state.len() == 1 {
        let last = state[0] != 0;
        state[0] = sum;
        partition[0] = sum;

        return last;
    }

    if state[0] >= sum {
        state[0] = 0;
        return true;
    }

    let prev_last = next_partition(sum - state[0], &mut partition[1..], &mut state[1..]);

    if prev_last {
        state[0] += 1;
        for i in 1..state.len() {
            state[i] = 0;
        }
        let prev_last = next_partition(sum - state[0], &mut partition[1..], &mut state[1..]);

        if prev_last {
            partition[0] = state[0];

            for i in 1..partition.len() {
                partition[i] = 0;
            }
            return false;
        }
    }

    partition[0] = state[0];

    false
}

fn num_partitions(sum: usize, n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return sum + 1;
    }
    if n == 3 {
        return sum + 1 + (sum + 1) * sum / 2;
    }

    let mut result = 0;

    for i in 0..=sum {
        result += num_partitions(sum - i, n - 1);
    }

    result
}

fn solve_rec(buttons: Vec<Vec<usize>>, target: Vec<usize>, max_presses: u64) -> Option<u64> {
    if target.iter().all(|c| *c == 0) {
        return Some(0);
    }
    if buttons.is_empty() {
        return None;
    }
    let (buttons, target) = sort(buttons, target);

    if target.last().is_some_and(|c| *c == 0) {
        let mut target = target;
        target.pop();
        let buttons = buttons
            .into_iter()
            .map(|b| b.into_iter().filter(|i| *i != target.len()).collect())
            .collect();
        return solve_rec(buttons, target, max_presses);
    }
    if target.last().is_some_and(|c| *c > max_presses as usize) {
        return None;
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

    let mut state = &mut [0; 10][0..affecting_buttons.len()];
    let mut p = &mut [0; 10][0..affecting_buttons.len()];

    'partitions: while !next_partition(sum, &mut p, &mut state) {
        let mut target_left = target.clone();

        for (i, presses) in p.iter().enumerate() {
            for b in &buttons[affecting_buttons[i]] {
                if target_left[*b] < *presses {
                    continue 'partitions;
                }

                target_left[*b] -= presses;
            }
        }

        let to_remove: Vec<_> = target_left
            .iter()
            .enumerate()
            .filter_map(|(i, c)| (*c == 0).then_some(i))
            .collect();

        let new_buttons = buttons
            .iter()
            .filter(|b| to_remove.iter().all(|r| !b.contains(r)))
            .cloned()
            .collect();
        debug_assert_eq!(target_left.last(), Some(&0));
        target_left.pop();

        if let Some(presses) = solve_rec(new_buttons, target_left, min_presses.saturating_sub(1)) {
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
            let mut buttons: Vec<Vec<_>> = rest[..rest.len() - 1]
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            buttons.sort_by_key(|b| b.len());
            let counters = rest.last().unwrap();
            let counters: Vec<usize> = counters[1..counters.len() - 1]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            (buttons, counters)
        })
        .collect();

    let mut total = 0;

    for (i, (buttons, target)) in input.into_iter().enumerate() {
        total += dbg!(solve_rec(buttons, target, u64::MAX).unwrap());
        dbg!(i, total);
    }

    Some(total)
}

fn sort(buttons: Vec<Vec<usize>>, target: Vec<usize>) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut counters: Vec<_> = target.into_iter().enumerate().collect();
    counters.sort_by_cached_key(|(i, c)| {
        Reverse({
            let sum = *c;
            let n = buttons
                .iter()
                .enumerate()
                .filter_map(|(j, b)| if b.contains(&i) { Some(j) } else { None })
                .count();

            num_partitions(sum, n)
        })
    });

    // dbg!(&buttons, &counters);
    let buttons = buttons
        .into_iter()
        .map(|b| {
            b.into_iter()
                .map(|b| counters.iter().position(|(i, _)| *i == b).unwrap())
                .collect()
        })
        .collect();

    let counters = counters.into_iter().map(|(_, c)| c).collect();
    (buttons, counters)
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
