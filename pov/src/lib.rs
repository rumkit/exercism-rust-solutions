use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;
use std::mem;

#[derive(Debug, PartialOrd, Ord, Eq)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    children: VecDeque<Tree<T>>
}

impl<T: Debug + Ord> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.children_eq(other)
    }
}


impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self { label, children: VecDeque::new() }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.push_back(child);
        self
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        if self.label == *from {
            return true;
        }
        let mut path = Vec::new();

        if dfs_path_nodes(self, from, &mut path) {
            while let Some(mut child) = path.pop() {
                mem::swap(self, &mut child);
                self.children.push_back(child);
            }
            return true;
        }
        false
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if from == to {
            return Some(Vec::new());
        }
        // change pov and then find the path from the root
        if self.pov_from(from) {
            let mut path = vec![from];
            if dfs_path_labels(&self.children, to, &mut path) {
                return Some(path);
            }
        }
        None
    }

    // comparing children order insensitive
    fn children_eq(&self, other: &Self) -> bool {
        fn counts<T: Ord>(v: &VecDeque<T>) -> BTreeMap<&T, usize> {
            let mut m = BTreeMap::new();
            for x in v {
                *m.entry(x).or_insert(0) += 1;
            }
            m
        }

        counts(&self.children) == counts(&other.children)
    }
}

fn dfs_path_nodes<T: Debug + Ord>(tree_node: &mut Tree<T>, new_root: & T, path:&mut Vec<Tree<T>>) -> bool {
    for _ in 0..tree_node.children.len() {
        let mut child = tree_node.children.pop_front().unwrap();
        if child.label == *new_root {
            path.push(child);
            return true;
        }
        if dfs_path_nodes(&mut child, new_root, path) {
            path.push(child);
            return true;
        }
        tree_node.children.push_back(child);
    }

    false
}


fn dfs_path_labels<'a, T: Debug + Ord>(children: &'a VecDeque<Tree<T>>, label: &'a T, path: &mut Vec<&'a T>) -> bool {
    for child in children {
        path.push(&child.label);
        if child.label == *label {
            return true;
        }
        if dfs_path_labels(&child.children, label, path) {
            return true;
        }
        path.pop();
    }
    false
}
