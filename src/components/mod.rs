use crate::prelude::*;

use std::collections::HashSet;

mod attributes;
pub use attributes::*;

mod commands;
pub use commands::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Actor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Opaque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Blocking;

#[derive(Debug, Clone, PartialEq)]
pub struct Render {
    pub name: String,
    pub tile: usize,
    pub pt: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self{
            visible_tiles: HashSet::new(),
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

#[derive(Debug, Clone, PartialEq)]
pub struct MightTalk {
    pub chance: i32, // in 1000
    pub phrases: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextDisplay {
    Fade(Point),
    AnimateUp(Point),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub display: TextDisplay,
    pub text: String,
    pub color: RGBA,
    pub ticks: i32,
    pub count: i32,
}

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

    pub fn next(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveStrategy {
    Random(usize),
    Patrol(usize, Direction),
    Chase(usize),
    Flee(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outlook {
    Aggressive,
    Neutral,
    Fearful,
}