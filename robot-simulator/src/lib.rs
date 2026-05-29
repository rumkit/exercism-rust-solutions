// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<i32> for Direction {
    fn from(value: i32) -> Direction {
        match value.rem_euclid(4) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("unknown direction: {}", value),
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Robot { d: Direction::from(self.d as i32 + 1), ..self }
    }

    pub fn turn_left(self) -> Self {
        Robot { d: Direction::from(self.d as i32 - 1), ..self }
    }

    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Robot { y: self.y + 1, ..self },
            Direction::East => Robot { x: self.x + 1, ..self },
            Direction::South => Robot { y: self.y - 1, ..self },
            Direction::West => Robot { x: self.x - 1, ..self },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, instruction|
            match instruction.to_ascii_uppercase() {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => panic!("unknown instruction: {}", instruction),
            })
}

pub fn position(&self) -> (i32, i32) {
    (self.x, self.y)
}

pub fn direction(&self) -> &Direction {
    &self.d
}
}
