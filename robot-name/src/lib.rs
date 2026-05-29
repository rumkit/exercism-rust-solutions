use rand::{Rng, RngExt};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    names_registry: Rc<RobotNamesRegistry>,
}

pub struct RobotNamesRegistry {
    taken_names: RefCell<HashSet<String>>,
}

pub struct Robot {
    name: String,
    names_registry: Rc<RobotNamesRegistry>,
}

impl RobotFactory {
    pub fn new() -> Self { RobotFactory { names_registry: Rc::new(RobotNamesRegistry::new()) }}

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let name = self.names_registry.claim_new_name(rng);
        Robot {
            name,
            names_registry: self.names_registry.clone(),
        }
    }
}

impl RobotNamesRegistry {
    pub fn new() -> Self { RobotNamesRegistry { taken_names: RefCell::new(HashSet::new()) }}

    pub fn claim_new_name<R: Rng>(&self, rng: &mut R) -> String {
        loop {
            let name = RobotNamesRegistry::generate_name(rng);
            if self.taken_names.borrow_mut().insert(name.clone()) {
                return name;
            }
        }
    }

    pub fn release_name(&self, name: &str) {
        self.taken_names.borrow_mut().remove(name);
    }

    fn generate_name<R: Rng>(rng: &mut R) -> String {
        format!(
            "{}{}{:03}",
            rng.random_range('A'..='Z'),
            rng.random_range('A'..='Z'),
            rng.random_range(0..=999)
        )
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        self.names_registry.release_name(&self.name);
        self.name = self.names_registry.claim_new_name(rng);
    }
}
