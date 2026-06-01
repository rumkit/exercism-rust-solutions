use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Hash + Eq + Clone> {
    internal: HashSet<T>,
}

impl<T> FromIterator<T> for CustomSet<T> where T: Hash + Eq + Clone {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        CustomSet { internal: HashSet::from_iter(iter) }
    }
}

impl<T> CustomSet<T> where T: Hash + Eq + Clone {
    pub fn new(input: &[T]) -> Self {
        input.iter()
            .cloned()
            .collect::<Self>()
    }

    pub fn contains(&self, element: &T) -> bool {
        self.internal.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.internal.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.internal.iter().all(|x| other.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.internal.iter().all(|x| !other.contains(x))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        self.internal.iter()
            .filter(|x| other.contains(x))
            .cloned()
            .collect()
    }

    pub fn difference(&self, other: &Self) -> Self {
        self.internal.iter()
            .filter(|x| !other.contains(x))
            .cloned()
            .collect()
    }

    pub fn union(&self, other: &Self) -> Self {
        self.internal.iter()
            .chain(other.internal.iter())
            .cloned()
            .collect()
    }
}
