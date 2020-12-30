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
    pub color: ColorPair,
    pub tile: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name(String);

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