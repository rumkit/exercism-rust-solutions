pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (1 << (ALLERGIES.iter().position(|r| *r == *allergen).unwrap() as u32)) > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGIES.iter()
            .enumerate()
            .filter(|&(i, _)| (1 << (i as u32)) & self.score > 0)
            .map(|(_,allergen)| allergen)
            .copied()
            .collect()
    }
}

const ALLERGIES: &[Allergen]= &[
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];
