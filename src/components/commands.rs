use crate::prelude::*;

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