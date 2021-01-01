use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FadingText {
    pub pt: Point,
    pub text: String,
    pub life: i32,
    pub remaining: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RandomMover(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn random() -> Self {
        let mut rng = Rng::new();
        match rng.range(0, 4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatrolMover(pub Direction);

impl PatrolMover {
    pub fn next(&self) -> PatrolMover {
        match self.0 {
            Direction::North => PatrolMover(Direction::East),
            Direction::East => PatrolMover(Direction::South),
            Direction::South => PatrolMover(Direction::West),
            Direction::West => PatrolMover(Direction::North),
        }
    }
}