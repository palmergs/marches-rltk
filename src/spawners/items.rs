use crate::prelude::*;

pub fn spawn_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Torch".to_string()),
            Item,
            pt,
            Render{ tile: tile_index(1, 11) },
            FieldOfLight::new(5),
        )
    );
}

pub fn spawn_chest(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Chest".to_string()),
            Item,
            pt,
            Render{ tile: tile_index(2, 7)},
        )
    );
}