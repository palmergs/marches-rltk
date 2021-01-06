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

pub fn spawn_dagger(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item::new(false, false),
            Render{
                name: "Dagger".to_string(),
                tile: tile_index(13, 116),
                pt,
            },
            Stats{
                armor: 0,
                speed: 0,
                vigor: Vigor::new(10),
                focus: Focus::new(0),
            },
        )
    );
}

pub fn spawn_mushroom_patch(ecs: &mut World, rng: &mut Rng, map: &Map, rect: &Rect) {
    for y in rect.y1..=rect.y2 {
        for x in rect.x1..=rect.x2 {
            let pt = Point::new(x, y);
            if map.can_enter(pt) {
                match rng.range(0, 4) {
                    0 => spawn_mushroom1(ecs, pt),
                    1 => spawn_mushroom2(ecs, pt),
                    _ => (),
                }
            }
        }
    }
}

pub fn spawn_mushroom1(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item::new(false, false),
            Render{
                name: "Mushroom".to_string(),
                tile: tile_index(12, 35),
                pt,
            },
            Stats{
                armor: 0,
                speed: 0,
                vigor: Vigor::new(1),
                focus: Focus::new(0),
            },
        )
    );
}

pub fn spawn_mushroom2(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item::new(false, false),
            Render{
                name: "Mushroom".to_string(),
                tile: tile_index(13, 35),
                pt,
            },
            Stats{
                armor: 0,
                speed: 0,
                vigor: Vigor::new(1),
                focus: Focus::new(0),
            },
        )
    );
}

pub fn spawn_chest(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item::new(true, false),
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

pub fn spawn_open_room_doors(ecs: &mut World, map: &Map, rect: &Rect) {
    find_door_locations(map, rect)
        .iter()
        .for_each(|pt| spawn_open_door(ecs, *pt)); 
}

pub fn spawn_closed_room_doors(ecs: &mut World, map: &Map, rect: &Rect) {
    find_door_locations(map, rect)
        .iter()
        .for_each(|pt| spawn_closed_door(ecs, *pt)); 
}

pub fn spawn_room_doors(ecs: &mut World, rng: &mut Rng, map: &Map, rect: &Rect) {
    find_door_locations(map, rect)
        .iter()
        .for_each(|pt| spawn_random_door(ecs, rng, *pt));
}

fn find_door_locations(map: &Map, rect: &Rect) -> Vec<Point> {
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

pub fn spawn_random_door(ecs: &mut World, rng: &mut Rng, pt: Point) {
    match rng.range(0, 2) {
        0 => spawn_closed_door(ecs, pt),
        _ => spawn_open_door(ecs, pt),
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