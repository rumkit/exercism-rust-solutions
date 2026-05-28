pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.contains(&0) {
            return None;
        }
        match sides {
            [a, b, c] if a + b >= c && b + c >= a && a + c >= b => Some(Triangle(a, b, c)),
            _ => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles() && !self.is_equilateral()
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.2 == self.0
    }
}
