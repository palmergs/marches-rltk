use crate::prelude::*;

mod items;
use items::*;

mod actors;
use actors::*;

pub fn spawn_player(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Player,
            Actor,
            Render{
                name: "Borimir".to_string(),
                tile: tile_index(2, 21),
                pt,
            },
            FieldOfView::new(10),
            FieldOfLight::new(5),
            Stats {
                armor: 0,
                speed: 2,
                vigor: Vigor::new(15),
                focus: Focus::new(15),
            },
            Physical{
                brawn: Brawn::new(0),
                grace: Grace::new(0),
            },
            Mental{
                outlook: Outlook::Player,
                strategy: MoveStrategy::Player,
                smart: Smart::new(0),
                charm: Charm::new(0),
            }
        )
    );
}

pub fn spawn_room_items(ecs: &mut World, rng: &mut Rng, map: &Map, rect: Rect, depth: i32) {
    match rng.range(0, 10 + depth) {
        0 => spawn_torch(ecs, Point::new(rect.x1, rect.y1)),
        1 => spawn_torch(ecs, Point::new(rect.x1, rect.y2)),
        2 => spawn_torch(ecs, Point::new(rect.x2, rect.y1)),
        3 => spawn_torch(ecs, Point::new(rect.x2, rect.y2)),
        4 => spawn_chest(ecs, Point::new(rng.range(rect.x1, rect.x2+1), rng.range(rect.y1, rect.y2+1))),
        5 => spawn_open_room_doors(ecs, map, &rect),
        6 => spawn_room_doors(ecs, rng, map, &rect),
        7 => spawn_closed_room_doors(ecs, map, &rect),
        8 => spawn_mushroom_patch(ecs, rng, map, &rect),
        _ => ()
    }
}

pub fn spawn_dropped_item(ecs: &mut World, rng: &mut Rng, map: &Map, depth: i32) {
    let mut tries = 10;
    loop {
        tries -= 1;
        if tries <= 0 {
            return;
        }

        let idx = rng.range(0, MAP_TILES);
        let pt = map.index_to_point2d(idx);
        if !map.can_enter(pt) {
            continue;
        }

        match rng.range(0 + depth, 14 + depth) {
            0..=3  => spawn_dagger(ecs, pt),
            10     => spawn_mushroom1(ecs, pt),
            11     => spawn_mushroom2(ecs, pt),
            _      => ()
        }
    }
}

pub fn spawn_monster(ecs: &mut World, rng: &mut Rng, pt: Point, depth: i32) {
    match rng.range(0 + depth, 15 + depth) {
        0..=2   => spawn_rat(ecs, pt),
        3..=5   => spawn_bat(ecs, pt),
        6..=8   => spawn_goblin_with_torch(ecs, pt),
        9..=10  => spawn_goblin(ecs, pt),
        11      => spawn_giant_rat(ecs, pt),
        12      => spawn_skeleton_with_torch(ecs, pt),
        13..=14 => spawn_skeleton(ecs, pt),
        15      => spawn_skeleton_warrior(ecs, pt),
        16      => spawn_animated_tree(ecs, pt),
        _       => spawn_bat(ecs, pt)
    }
}

