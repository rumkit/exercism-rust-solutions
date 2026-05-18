use std::collections::{BTreeMap, BTreeSet, HashSet};

pub struct School<'a> {
    roster: BTreeMap<u32, BTreeSet<&'a str>>,
    students: HashSet<&'a str>,
    empty: BTreeSet<&'a str>,
}

impl<'a> School<'a> {
    pub fn new() -> Self {
        School {
            roster: BTreeMap::new(),
            students: HashSet::new(),
            empty: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        if self.students.contains(student) {
            println!("Student record for: {} already exists.", student);
        } else {
            self.students.insert(student);
            let grade = self.roster.entry(grade).or_default();
            grade.insert(student);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .unwrap_or(&self.empty)
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
}
