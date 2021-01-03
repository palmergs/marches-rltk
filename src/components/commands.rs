use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct WantsToMove {
    pub actor: Entity,
    pub destination: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WantsToAttack {
    pub actor: Entity,
    pub victim: Entity,
    // pub power: i32,
    // pub dmg: i32,
}