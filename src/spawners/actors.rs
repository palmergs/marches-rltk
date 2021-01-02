use crate::prelude::*;

pub fn spawn_rat(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Dungeon Rat".to_string()),
            Actor,
            pt,
            Render{ tile: tile_index(13, 1) },
            FieldOfView::new(4),
            MightTalk{ chance: 20, phrase: "squeek!".to_string() },
            RandomMover(2),
        )
    );
}

pub fn spawn_bat(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Dungeon Rat".to_string()),
            Actor,
            pt,
            Render{ tile: tile_index(12, 19) },
            FieldOfView::new(4),
            RandomMover(1),
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
            MightTalk{ chance: 1, phrase: "Haroom!".to_string() },
            RandomMover(20),
        )
    );
}

pub fn spawn_goblin_with_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Goblin with torch".to_string()),
            Actor,
            pt,
            Render{ tile: tile_index(12, 9) },
            FieldOfView::new(7),
            FieldOfLight::new(5),
            PatrolMover(Direction::random()),
        )
    );
}

pub fn spawn_goblin(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Goblin".to_string()),
            Actor,
            pt,
            Render{ tile: tile_index(12, 9) },
            FieldOfView::new(7),
            PatrolMover(Direction::random()),
        )
    );
}

pub fn spawn_skeleton_with_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Skeleton with torch".to_string()),
            Actor,
            pt,
            Render{ tile: tile_index(12, 23) },
            FieldOfView::new(5),
            FieldOfLight::new(5),
            PatrolMover(Direction::random()),
        )
    );
}

pub fn spawn_skeleton(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Name("Skeleton".to_string()),
            Actor,
            pt,
            Render{ tile: tile_index(12, 23) },
            FieldOfView::new(5),
            PatrolMover(Direction::random()),
        )
    );
}