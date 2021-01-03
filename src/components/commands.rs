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
}

#[derive(Debug, Clone, PartialEq)]
pub struct WantsToChangeStrategy {
    pub actor: Entity,
    pub strategy: MoveStrategy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WantsToChangeOutlook {
    pub actor: Entity,
    pub outlook: Outlook,
}