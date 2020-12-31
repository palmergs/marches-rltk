use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Player,
            Actor,
            pt,
            Render{ tile: tile_index(2, 21) },
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
            Render{ tile: tile_index(1, 11) },
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
            Render{ tile: tile_index(13, 1) },
            FieldOfView::new(6),
        )
    );
}

pub fn spawn_animated_tree(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Animated Tree".to_string()),
            Actor,
            pt,
            Render{ tile: tile_index(1, 22) },
            FieldOfView::new(7),
            FieldOfLight::new(5),
        )
    );
}

pub fn spawn_goblin_with_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Goblin with torch".to_string()),
            Actor,
            pt,
            Render{ tile: tile_index(2, 22) },
            FieldOfView::new(7),
            FieldOfLight::new(5),
        )
    );
}

fn tile_index(row: usize, col: usize) -> usize {
    ((row - 1) * 128) + (col - 1)
}