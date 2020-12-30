use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Player,
            Actor,
            pt,
            Render{ tile: 128 + 20 },
            FieldOfView::new(10),
            FieldOfLight::new(5),
        )
    );
}

pub fn spawn_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Torch".to_string()),
            Item,
            pt,
            Render{ tile: 10 },
            FieldOfLight::new(5),
        )
    );
}

pub fn spawn_rat(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Dungeon Rat".to_string()),
            Actor,
            pt,
            Render{ tile: (17*128) + 0 },
            FieldOfView::new(6),
        )
    );
}

pub fn spawn_goblin_with_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Goblin".to_string()),
            Actor,
            pt,
            Render{ tile: (16*128) + 8 },
            FieldOfView::new(7),
            FieldOfLight::new(5),
        )
    );
}