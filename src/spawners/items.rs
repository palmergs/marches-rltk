use crate::prelude::*;

pub fn spawn_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item,
            Render{ 
                name: "Torch".to_string(),
                tile: tile_index(1, 11),
                pt
            },
            FieldOfLight::new(5),
        )
    );
}

pub fn spawn_chest(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item,
            Render{ 
                name: "Closed Chest".to_string(),
                tile: tile_index(2, 7),
                pt,
            },
        )
    );
}