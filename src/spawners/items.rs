use crate::prelude::*;

pub fn spawn_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item::new(false, false),
            Render{
                name: "Torch".to_string(),
                tile: tile_index(1, 11),
                pt
            },
            FieldOfLight::new(5),
            Stats{
                armor: 0,
                speed: 0,
                vigor: Vigor::new(3),
                focus: Focus::new(0),
            },
        )
    );
}

pub fn spawn_chest(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item::new(false, false),
            Render{
                name: "Closed Chest".to_string(),
                tile: tile_index(2, 7),
                pt,
            },
            Stats{
                armor: 5,
                speed: 0,
                vigor: Vigor::new(30),
                focus: Focus::new(0),
            },
        )
    );
}

pub fn spawn_random_door(ecs: &mut World, rng: &mut Rng, pt: Point) {
    match rng.range(0, 3) {
        0 | 1 => spawn_open_door(ecs, pt),
        _ => spawn_closed_door(ecs, pt),
    }
}

pub fn spawn_open_door(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item::new(false, false),
            Render{
                name: "Door".to_string(),
                tile: tile_index(1, 4),
                pt,
            },
            Stats{
                armor: 3,
                speed: 0,
                vigor: Vigor::new(30),
                focus: Focus::new(0),
            },
        )
    );
}

pub fn spawn_closed_door(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item::new(true, true),
            Render{
                name: "Door".to_string(),
                tile: tile_index(1, 3),
                pt,
            },
            Stats{
                armor: 3,
                speed: 0,
                vigor: Vigor::new(30),
                focus: Focus::new(0),
            },
        )
    );
}