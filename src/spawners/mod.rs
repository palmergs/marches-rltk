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
    match rng.range(0, 8) {
        0 => spawn_torch(ecs, Point::new(rect.x1, rect.y1)),
        1 => spawn_torch(ecs, Point::new(rect.x1, rect.y2)),
        2 => spawn_torch(ecs, Point::new(rect.x2, rect.y1)),
        3 => spawn_torch(ecs, Point::new(rect.x2, rect.y2)),
        4 => spawn_chest(ecs, Point::new(rng.range(rect.x1, rect.x2+1), rng.range(rect.y1, rect.y2+1))),
        5 => find_door_locations(map, rect).iter().for_each(|pt| spawn_random_door(ecs, rng, *pt)),
        6 => find_door_locations(map, rect).iter().for_each(|pt| spawn_closed_door(ecs, *pt)),
        7 => find_door_locations(map, rect).iter().for_each(|pt| spawn_open_door(ecs, *pt)),
        _ => ()
    }
}

fn find_door_locations(map: &Map, rect: Rect) -> Vec<Point> {
    let mut vec = Vec::new();
    for x in rect.x1 ..= rect.x2 {
        if rect.y1 > 0 {
            let pt = Point::new(x, rect.y1 - 1);
            if map.is_passage(pt) { vec.push(pt); }
        }

        if rect.y2 < MAP_HEIGHT as i32 - 1 {
            let pt = Point::new(x, rect.y2 + 1);
            if map.is_passage(pt) { vec.push(pt); }
        }
    } 

    for y in rect.y1 ..= rect.y2 {
        if rect.x1 > 0 {
            let pt = Point::new(rect.x1 - 1, y);
            if map.is_passage(pt) { vec.push(pt); }
        }

        if rect.x2 < MAP_WIDTH as i32 - 1 {
            let pt = Point::new(rect.x2 + 1, y);
            if map.is_passage(pt) { vec.push(pt); }
        }
    }
    vec
}

pub fn spawn_dropped_item(ecs: &mut World, rng: &mut Rng, map: &Map, depth: i32) {

}

pub fn spawn_monster(ecs: &mut World, rng: &mut Rng, pt: Point, depth: i32) {
    match rng.range(0 + depth, 14 + depth) {
        0..=3   => spawn_rat(ecs, pt),
        4..=5   => spawn_goblin_with_torch(ecs, pt),
        6..=8   => spawn_goblin(ecs, pt),
        9       => spawn_skeleton_with_torch(ecs, pt),
        10..=12 => spawn_skeleton(ecs, pt),
        13      => spawn_animated_tree(ecs, pt),
        _       => spawn_bat(ecs, pt)
    }
}

