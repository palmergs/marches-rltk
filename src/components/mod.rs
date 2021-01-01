pub use crate::prelude::*;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Actor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Render {
    pub tile: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct WantsToMove {
    pub actor: Entity,
    pub destination: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WantsToInteract {
    pub actor: Entity,
    pub victim: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MightTalk {
    pub chance: i32, // in 1000
    pub phrase: String,
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub visited_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self{
            visible_tiles: HashSet::new(),
            visited_tiles: HashSet::new(),
            radius,
            is_dirty: true
        }
    }

    pub fn clone_dirty(&self) -> Self {
        let mut cloned = self.clone();
        cloned.is_dirty = true;
        cloned
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfLight {
    pub lit_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfLight {
    pub fn new(radius: i32) -> Self {
        Self{ lit_tiles: HashSet::new(), radius, is_dirty: true }
    }

    pub fn clone_dirty(&self) -> Self {
        let mut cloned = self.clone();
        cloned.is_dirty = true;
        cloned
    }
}