use crate::prelude::*;

mod items;
mod monsters;

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

pub fn spawn_items(ecs: &mut World, rng: &mut Rng, rect: Rect, depth: i32) {
    match rng.range(0, 8) {
        0 => spawn_torch(ecs, Point::new(rect.x1, rect.y1)),
        1 => spawn_torch(ecs, Point::new(rect.x1, rect.y2)),
        2 => spawn_torch(ecs, Point::new(rect.x2, rect.y1)),
        3 => spawn_torch(ecs, Point::new(rect.x2, rect.y2)),
        _ => ()
    }
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

pub fn spawn_monster(ecs: &mut World, rng: &mut Rng, pt: Point, depth: i32) {
    match rng.range(0 + depth, 10 + depth) {
        0..=4 => spawn_rat(ecs, pt),
        6     => spawn_skeleton_with_torch(ecs, pt),
        7..=9 => spawn_skeleton(ecs, pt),
        10    => spawn_animated_tree(ecs, pt),
        _     => spawn_bat(ecs, pt)

    }
}

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

fn tile_index(row: usize, col: usize) -> usize {
    ((row - 1) * 128) + (col - 1)
}