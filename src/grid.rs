use std::{
    ops::{Index, IndexMut},
    vec::IntoIter,
};

#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_filled(width: usize, height: usize, t: T) -> Self {
        Grid {
            data: vec![t; width * height],
            width,
            height,
        }
    }

    pub fn from_vec_rows(vecs: &[Vec<T>]) -> Option<Self> {
        let height = vecs.len();

        let Some(width) = vecs.get(0).map(|v| v.len()) else {
            return None;
        };

        for row in vecs {
            if row.len() != width {
                return None;
            }
        }

        let data: Vec<_> = vecs.iter().flat_map(|v| v.iter()).cloned().collect();

        Some(Self {
            data,
            width,
            height,
        })
    }

    pub fn rotated_clockwise(&self) -> Grid<T> {
        let mut new_data = Vec::with_capacity(self.data.len());

        for row in 0..self.height {
            for col in (0..self.width).rev() {
                new_data.push(self.get((col, row)).unwrap().clone());
            }
        }

        Grid {
            data: new_data,
            width: self.height,
            height: self.width,
        }
    }

    pub fn rotated_counterclockwise(&self) -> Grid<T> {
        let mut new_data = Vec::with_capacity(self.data.len());

        for row in (0..self.height).rev() {
            for col in 0..self.width {
                new_data.push(self.get((col, row)).unwrap().clone());
            }
        }

        Grid {
            data: new_data,
            width: self.height,
            height: self.width,
        }
    }
}

impl<T> Grid<T> {
    pub fn parse<F: Fn(char) -> T>(input: &str, f: F) -> Option<Self> {
        let mut data = Vec::with_capacity(input.len());
        let mut width = 0;
        let mut height = 0;

        for (i, line) in input
            .lines()
            .enumerate()
            .take_while(|(_, line)| !line.is_empty())
        {
            if i == 0 {
                width = line.len()
            } else if width != line.len() {
                return None;
            }

            for char in line.chars() {
                data.push(f(char));
            }

            height = i + 1;
        }

        Some(Grid {
            data,
            width,
            height,
        })
    }

    pub fn from_data(data: Vec<T>, width: usize) -> Option<Self> {
        (data.len() % width == 0).then(|| Grid {
            height: data.len() / width,
            data,
            width,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn area(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, (row, col): (usize, usize)) -> Option<&T> {
        if row >= self.height || col >= self.width {
            return None;
        }

        self.data.get(row * self.width + col)
    }

    pub fn get_mut(&mut self, (row, col): (usize, usize)) -> Option<&mut T> {
        if row >= self.height || col >= self.width {
            return None;
        }

        self.data.get_mut(row * self.width + col)
    }

    pub fn get_row(&self, row: usize) -> &[T] {
        &self.data[row * self.width..(row + 1) * self.width]
    }

    pub fn set(&mut self, (row, col): (usize, usize), t: T) {
        if row >= self.height || col >= self.width {
            return;
        }

        self.data[row * self.width + col] = t;
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height).flat_map(|row| (0..self.width).map(move |col| (row, col)))
    }

    pub fn position_values(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        (0..self.height).flat_map(move |row| {
            (0..self.width).map(move |col| ((row, col), self.get((row, col)).unwrap()))
        })
    }

    pub fn neighbours4(
        &self,
        (row, col): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..4).filter_map(move |i| match i {
            0 => (row > 0).then(|| (row - 1, col)),
            1 => (col > 0).then(|| (row, col - 1)),
            2 => (row < self.width - 1).then(|| (row + 1, col)),
            3 => (col < self.height - 1).then(|| (row, col + 1)),
            _ => unreachable!(),
        })
    }

    pub fn neighbours8(
        &self,
        (row, col): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..8).filter_map(move |i| match i {
            0 => (row > 0 && col > 0).then(|| (row - 1, col - 1)),
            1 => (row > 0).then(|| (row - 1, col)),
            2 => (row > 0 && col < self.height - 1).then(|| (row - 1, col + 1)),

            3 => (col > 0).then(|| (row, col - 1)),

            4 => (col < self.height - 1).then(|| (row, col + 1)),

            5 => (row < self.width - 1 && col > 0).then(|| (row + 1, col - 1)),
            6 => (row < self.width - 1).then(|| (row + 1, col)),
            7 => (row < self.width - 1 && col < self.height - 1).then(|| (row + 1, col + 1)),
            _ => unreachable!(),
        })
    }

    pub fn floodfill<F: Fn(&T) -> bool>(
        &self,
        start: (usize, usize),
        f: F,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut visited = vec![false; self.data.len()];

        let mut stack = vec![start];
        while let Some(pos) = stack.pop() {
            visited[pos.0 * self.width + pos.1] = true;

            for neighbour in self.neighbours4(pos) {
                if f(&self[neighbour]) && !visited[neighbour.0 * self.width + neighbour.1] {
                    stack.push(neighbour);
                }
            }
        }

        visited
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(i, _)| (i / self.width, i % self.width))
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Grid<U> {
        Grid {
            data: self.data.into_iter().map(f).collect(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn print<F: Fn(&T) -> char>(&self, f: F) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", f(&self[(i, j)]));
            }
            println!()
        }
    }

    pub fn print_path<F: Fn(&T) -> char>(&self, path: &[(usize, usize)], f: F) {
        for i in 0..self.height {
            for j in 0..self.width {
                if path.contains(&(i, j)) {
                    print!("O");
                } else {
                    print!("{}", f(&self[(i, j)]));
                }
            }
            println!()
        }
    }
}

impl<T> Grid<T>
where
    T: Eq,
{
    pub fn find(&self, value: T) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.data
            .iter()
            .enumerate()
            .filter(move |(_, t)| **t == value)
            .map(|(i, _)| (i / self.width, i % self.width))
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.get_row(index)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}
