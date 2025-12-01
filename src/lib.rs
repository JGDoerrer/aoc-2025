use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

pub mod template;

// Use this file to add helper functions and additional modules.
pub mod counting_set;
pub mod grid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Direction4 {
    Up,
    Down,
    Left,
    Right,
}

impl Direction4 {
    pub fn shift(&self, pos: (usize, usize)) -> (usize, usize) {
        self.shift_amount(pos, 1)
    }

    pub fn shift_amount(&self, pos: (usize, usize), amount: usize) -> (usize, usize) {
        match self {
            Direction4::Up => (pos.0 - amount, pos.1),
            Direction4::Down => (pos.0 + amount, pos.1),
            Direction4::Left => (pos.0, pos.1 - amount),
            Direction4::Right => (pos.0, pos.1 + amount),
        }
    }

    pub fn turn_left(&self) -> Direction4 {
        match self {
            Direction4::Up => Direction4::Left,
            Direction4::Down => Direction4::Right,
            Direction4::Left => Direction4::Down,
            Direction4::Right => Direction4::Up,
        }
    }

    pub fn turn_right(&self) -> Direction4 {
        match self {
            Direction4::Up => Direction4::Right,
            Direction4::Down => Direction4::Left,
            Direction4::Left => Direction4::Up,
            Direction4::Right => Direction4::Down,
        }
    }
}

impl From<char> for Direction4 {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction4::Left,
            '>' => Direction4::Right,
            '^' => Direction4::Up,
            'v' => Direction4::Down,
            _ => unreachable!(),
        }
    }
}

pub fn pathfind_cost<State, CostFn, Neighbours, End, I>(
    start: State,
    end: End,
    cost: CostFn,
    neighbours: Neighbours,
) -> Option<usize>
where
    State: Ord + Hash + Clone,
    CostFn: Fn(&State, &State) -> usize,
    Neighbours: Fn(&State) -> I,
    End: Fn(&State) -> bool,
    I: Iterator<Item = State>,
{
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));
    let mut visited = HashSet::new();

    while let Some((Reverse(current_cost), state)) = queue.pop() {
        if end(&state) {
            return Some(current_cost);
        }

        for n in neighbours(&state).filter(|n| !visited.contains(n)) {
            if visited.contains(&n) {
                continue;
            }

            queue.push((Reverse(current_cost + cost(&state, &n)), n));
        }

        visited.insert(state);
    }

    None
}

pub fn pathfind_predecessors<State, CostFn, Neighbours, End, I>(
    start: State,
    end: End,
    cost: CostFn,
    neighbours: Neighbours,
) -> Option<HashMap<State, Vec<State>>>
where
    State: Ord + Hash + Clone,
    CostFn: Fn(&State, &State) -> usize,
    Neighbours: Fn(&State) -> I,
    End: Fn(&State) -> bool,
    I: Iterator<Item = State>,
{
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start.clone()));
    let mut visited = HashSet::new();
    let mut predecessors: HashMap<State, Vec<State>> = HashMap::new();

    while let Some((Reverse(current_cost), state)) = queue.pop() {
        if end(&state) {
            return Some(predecessors);
        }

        for n in neighbours(&state).filter(|n| !visited.contains(n)) {
            queue.push((Reverse(current_cost + cost(&state, &n)), n.clone()));

            if let Some(v) = predecessors.get_mut(&n) {
                if !v.contains(&state) {
                    v.push(state.clone());
                }
            } else {
                predecessors.insert(n, vec![state.clone()]);
            }
        }

        visited.insert(state);
    }

    None
}

pub fn a_star_cost<State, CostFn, HFn, Neighbours, End, I>(
    start: State,
    end: End,
    cost: CostFn,
    h: HFn,
    neighbours: Neighbours,
) -> Option<usize>
where
    State: Ord + Hash + Clone,
    CostFn: Fn(&State, &State) -> usize,
    HFn: Fn(&State) -> usize,
    Neighbours: Fn(&State) -> I,
    End: Fn(&State) -> bool,
    I: Iterator<Item = State>,
{
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(h(&start)), start));
    let mut visited = HashSet::new();

    while let Some((Reverse(current_cost), state)) = queue.pop() {
        if end(&state) {
            return Some(current_cost - h(&state));
        }

        for n in neighbours(&state).filter(|n| !visited.contains(n)) {
            if visited.contains(&n) {
                continue;
            }

            queue.push((
                Reverse(current_cost + cost(&state, &n) - h(&state) + h(&n)),
                n,
            ));
        }

        visited.insert(state);
    }

    None
}
