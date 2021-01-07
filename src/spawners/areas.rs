use crate::prelude::*;

pub fn spawn_map_items(ecs: &mut World, rng: &mut Rng, map: &Map, count: usize, depth: i32) -> Vec<Entity> {
    let mut vec = Vec::new();
    let mut tries = count;
    loop {
        tries -= 1;
        if tries <= 0 {
            return vec;
        }

        let idx = rng.range(0, MAP_TILES);
        let pt = map.index_to_point2d(idx);
        if !map.can_enter(pt) {
            continue;
        }

        if let Some(e) = spawn_item(ecs, rng, pt, depth) {
            vec.push(e);
        }
    }
}

pub fn spawn_room_items(ecs: &mut World, rng: &mut Rng, map: &Map, rect: Rect, depth: i32) -> Vec<Entity> {
    match rng.range(0, 10 + depth) {
        0..=3 => spawn_room_torches(ecs, rng, rect),
        4 => spawn_open_room_doors(ecs, map, &rect),
        5 => spawn_room_doors(ecs, rng, map, &rect),
        6 => spawn_closed_room_doors(ecs, map, &rect),
        7 => spawn_mushroom_patch(ecs, rng, map, &rect),
        _ => vec![]
    }
}

// There's a 25% chance that a torch will be placed in each corner of the room
pub fn spawn_room_torches(ecs: &mut World, rng: &mut Rng, rect: Rect) -> Vec<Entity> {
    let mut vec = Vec::new();
    if rng.range(0, 4) == 0 {
        if let Some(e) = spawn("torch", ecs, Point::new(rect.x1, rect.y1)) {
            vec.push(e);
        }
    }

    if rng.range(0, 4) == 0 {
        if let Some(e) = spawn("torch", ecs, Point::new(rect.x2, rect.y1)) {
            vec.push(e);
        }
    }
    
    if rng.range(0, 4) == 0 {
        if let Some(e) = spawn("torch", ecs, Point::new(rect.x1, rect.y2)) {
            vec.push(e);
        }
    }
    
    if rng.range(0, 4) == 0 {
        if let Some(e) = spawn("torch", ecs, Point::new(rect.x2, rect.y2)) {
            vec.push(e);
        }
    }    

    vec
}

pub fn spawn_open_room_doors(ecs: &mut World, map: &Map, rect: &Rect) -> Vec<Entity> {
    let mut vec = Vec::new();
    find_door_locations(map, rect)
        .iter()
        .for_each(|pt| {
            if let Some(entity) = spawn("open door", ecs, *pt) {
                vec.push(entity);
            }
        }); 
    vec
}

pub fn spawn_closed_room_doors(ecs: &mut World, map: &Map, rect: &Rect) -> Vec<Entity> {
    let mut vec = Vec::new();
    find_door_locations(map, rect)
        .iter()
        .for_each(|pt| {
            if let Some(entity) = spawn("closed door", ecs, *pt) {
                vec.push(entity);
            }
        });
    vec 
}

pub fn spawn_room_doors(ecs: &mut World, rng: &mut Rng, map: &Map, rect: &Rect) -> Vec<Entity> {
    let mut vec = Vec::new();
    find_door_locations(map, rect)
        .iter()
        .for_each(|pt| {
            if let Some(entity) = spawn_random_door(ecs, rng, *pt) {
                vec.push(entity);
            }
        });
    vec
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

pub fn spawn_random_door(ecs: &mut World, rng: &mut Rng, pt: Point) -> Option<Entity> {
    match rng.range(0, 2) {
        0 => spawn("closed door", ecs, pt),
        _ => spawn("open door", ecs, pt),
    }
}

pub fn spawn_mushroom_patch(ecs: &mut World, rng: &mut Rng, map: &Map, rect: &Rect) -> Vec<Entity> {
    let mut vec = Vec::new();
    for y in rect.y1..=rect.y2 {
        for x in rect.x1..=rect.x2 {
            let pt = Point::new(x, y);
            if map.can_enter(pt) {
                let _ = match rng.range(0, 4) {
                    0 => if let Some(e) = spawn("mushroom1", ecs, pt) { vec.push(e); },
                    1 => if let Some(e) = spawn("mushroom2", ecs, pt) { vec.push(e); },
                    _ => (),
                };
            }
        }
    }
    vec
}