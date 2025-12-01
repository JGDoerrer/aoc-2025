use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct CountingSet<T>(HashMap<T, usize>);

impl<T> CountingSet<T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        CountingSet(HashMap::new())
    }

    pub fn insert(&mut self, t: T) {
        self.insert_count(t, 1);
    }

    pub fn insert_count(&mut self, t: T, count: usize) {
        if let Some(c) = self.0.get_mut(&t) {
            *c += count;
        } else {
            self.0.insert(t, count);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> IntoIterator for CountingSet<T> {
    type Item = (T, usize);
    type IntoIter = <HashMap<T, usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> FromIterator<T> for CountingSet<T>
where
    T: Hash + Eq,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut set = Self::new();

        for i in iter {
            set.insert(i);
        }

        set
    }
}

impl<T> FromIterator<(T, usize)> for CountingSet<T>
where
    T: Hash + Eq,
{
    fn from_iter<U: IntoIterator<Item = (T, usize)>>(iter: U) -> Self {
        let mut set = Self::new();

        for (i, c) in iter {
            set.insert_count(i, c);
        }

        set
    }
}
