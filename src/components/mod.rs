use crate::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

mod attributes;
pub use attributes::*;

mod commands;
pub use commands::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub depth: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    // hard coded id of this item
    pub id: String,

    // if true this item blocks movement if on the map
    pub blocking: bool,

    // if true this item blocks vision if on the map
    pub opaque: bool,

    // if true this item can be picked up (and dropped)
    // to add to inventory
    pub can_get: bool,
}

impl Item {
    pub fn is_blocking(&self) -> bool { self.blocking }
    pub fn is_opaque(&self) -> bool { self.opaque }
    pub fn is_carryable(&self) -> bool { self.can_get }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Actor;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Render {
    pub name: String,
    pub tile: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Carried;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoresVigor {
    pub amount: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoresFocus {
    pub amount: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consumable{
    pub count: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equippable {
    pub primary: EquipmentSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Head,
    Neck,
    RightRing,
    LeftRing,
    RightHand,
    LeftHand,
    BothHands,
    Belt,
    Feet,
    Body,
    Shoulders,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equipped {
    pub slot: EquipmentSlot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpawnTrigger {
    Killed,
    Opened,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntity {
    pub id: String,
    pub chance: i32, // out of 1000
    pub trigger: SpawnTrigger,
}

impl SpawnEntity {
    pub fn new(id: &str, chance: i32, trigger: SpawnTrigger) -> SpawnEntity {
        SpawnEntity{ id: id.to_string(), chance, trigger }
    }

    pub fn should_spawn(&self, rng: &mut Rng) -> bool {
        let rolled = rng.range(0, 1000);
        rolled < self.chance
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spawns {
    pub entities: Vec<SpawnEntity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stairs {
    pub to_depth: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MightTalk {
    pub chance: i32, // in 1000
    pub phrases: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextDisplay {
    Fade(Point),
    AnimateUp(Point),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub display: TextDisplay,
    pub text: String,
    pub color: RGBA,
    pub ticks: i32,
    pub count: i32,
}

impl Text {
    pub fn is_fade(&self) -> bool {
        match self.display {
            TextDisplay::Fade(_) => true,
            _ => false,
        }
    }
    pub fn pt(&self) -> Point {
        match self.display {
            TextDisplay::Fade(pt) => pt,
            TextDisplay::AnimateUp(pt) => pt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MoveStrategy {
    Player,
    Random,
    Patrol(Direction),
    Chase,
    Flee,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Outlook {
    Player,
    Aggressive,
    Neutral,
    Fearful,
}
