use crate::prelude::*;

mod items;
pub use items::*;

mod actors;
pub use actors::*;

mod areas;
pub use areas::*;

mod templates;

pub fn load_actors() {
    let actors = templates::Actors::load();
    println!("actors are {:?}", actors);
}

pub fn load_items() {
    let items = templates::Items::load();
    println!("items are {:?}", items);
}

pub fn spawn_player(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Player{
                depth: 0,
            },
            Actor,
            pt,
            Render{
                name: "Borimir".to_string(),
                tile: tile_index(2, 21),
            },
            FieldOfView::new(24),
            FieldOfLight::new(1),
            Stats {
                armor: 0,
                speed: 2,
                power: 0,
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

pub fn spawn(id: &str, ecs: &mut World, pt: Point) -> Option<Entity> {
    let entity = match id {
        "torch" =>          ecs.push(torch_tuple(pt)),
        "dagger" =>         ecs.push(dagger_tuple(pt)),
        "buckler" =>        ecs.push(buckler_tuple(pt)),
        "open door" =>      ecs.push(open_door_tuple(pt)),
        "closed door" =>    ecs.push(closed_door_tuple(pt)),
        "chest" =>          ecs.push(chest_tuple(pt)),
        "mushroom1" =>      ecs.push(mushroom1_tuple(pt)),
        "mushroom2" =>      ecs.push(mushroom2_tuple(pt)),
        "flaming sword" =>  ecs.push(flaming_sword_tuple(pt)),
        "ring of protection" => ecs.push(ring_of_protection_tuple(pt)),
        "ring of power" =>  ecs.push(ring_of_power_tuple(pt)),
        "ring of lesser radiance" => ecs.push(ring_of_lesser_radiance_tuple(pt)),
        "ring of greater radiance" => ecs.push(ring_of_greater_radiance_tuple(pt)),
        "seltzer" =>        ecs.push(seltzer_tuple(pt)),
        "healing potion" => ecs.push(healing_potion_tuple(pt)),

        "rat" =>            ecs.push(rat_tuple(pt)),
        "bat" =>            ecs.push(bat_tuple(pt)),
        "giant rat" =>      ecs.push(giant_rat_tuple(pt)),
        "doormouse" =>      ecs.push(doormouse_tuple(pt)),

        "skeleton" =>       ecs.push(skeleton_tuple(pt)),
        "skeleton with torch" => ecs.push(skeleton_with_torch_tuple(pt)),
        "skeleton warrior" => ecs.push(skeleton_warrior_tuple(pt)),
        "goblin" =>         ecs.push(goblin_tuple(pt)),
        "goblin with torch" => ecs.push(goblin_with_torch_tuple(pt)),
        "animated tree" => ecs.push(animated_tree_tuple(pt)),
        _ => return None,
    };
    Some(entity)
}

pub fn spawn_monster(ecs: &mut World, rng: &mut Rng, pt: Point, depth: i32) -> Option<Entity> {
    match rng.range(0 + depth, 15 + depth) {
        0..=2   => spawn("rat", ecs, pt), 
        3..=5   => spawn("bat", ecs, pt), 
        6..=8   => spawn("giant rat", ecs, pt), 
        9..=10  => spawn("skeleton", ecs, pt), 
        11      => spawn("skelton with torch", ecs, pt), 
        12      => spawn("skeleton warrior", ecs, pt), 
        13..=14 => spawn("goblin", ecs, pt), 
        15      => spawn("goblin with torch", ecs, pt), 
        16      => spawn("animated tree", ecs, pt), 
        _       => None,
    }
}

pub fn spawn_item(ecs: &mut World, rng: &mut Rng, pt: Point, depth: i32) -> Option<Entity> {
    match rng.range(0 + depth, 20 + depth) {
        0..=3   => spawn("dagger", ecs, pt),
        4..=5   => spawn("buckler", ecs, pt),
        6..=8   => spawn("torch", ecs, pt),
        9       => spawn("chest", ecs, pt),
        10      => spawn("mushroom1", ecs, pt),
        11      => spawn("mushroom2", ecs, pt),
        12..=15 => spawn("selzer", ecs, pt),
        16..=17 => spawn("healing potion", ecs, pt),
        18      => spawn("ring of protection", ecs, pt),
        19      => spawn("ring of lesser radiance", ecs, pt),
        20      => spawn("ring of power", ecs, pt),
        98      => spawn("ring of greater radiance", ecs, pt),
        99      => spawn("flaming sword", ecs, pt),
        _       => None
    }
}

pub fn spawn_room_stairs_up(ecs: &mut World, rng: &mut Rng, map: &Map, rect: &Rect, to_depth: i32) {
    let mut tries = 10;
    loop {
        tries -= 1;
        if tries == 0 {
            spawn_map_stairs_up(ecs, rng, map, to_depth);
            return
        }
        let x = rng.range(rect.x1, rect.x2 + 1);
        let y = rng.range(rect.y1, rect.y2 + 1);
        let pt = Point::new(x, y);
        if map.can_enter(pt) {
            ecs.push(stairs_up_tuple(pt, to_depth));
            return
        }
    }
}

pub fn spawn_map_stairs_up(ecs: &mut World, rng: &mut Rng, map: &Map, to_depth: i32) {
    let mut tries = 10;
    loop {
        tries -= 1;
        if tries == 0 {
            return
        }

        let x = rng.range(1, MAP_WIDTH - 2);
        let y = rng.range(1, MAP_HEIGHT - 2);
        let pt = Point::new(x, y);
        if map.can_enter(pt) {
            ecs.push(stairs_up_tuple(pt, to_depth));
            return
        }
    }
}

pub fn spawn_room_stairs_down(ecs: &mut World, rng: &mut Rng, map: &Map, rect: &Rect, to_depth: i32) {
    let mut tries = 10;
    loop {
        tries -= 1;
        if tries == 0 {
            spawn_map_stairs_down(ecs, rng, map, to_depth);
            return
        }
        let x = rng.range(rect.x1, rect.x2 + 1);
        let y = rng.range(rect.y1, rect.y2 + 1);
        let pt = Point::new(x, y);
        if map.can_enter(pt) {
            ecs.push(stairs_down_tuple(pt, to_depth));
            return
        }
    }
}

pub fn spawn_map_stairs_down(ecs: &mut World, rng: &mut Rng, map: &Map, to_depth: i32) {
    let mut tries = 10;
    loop {
        tries -= 1;
        if tries == 0 {
            return
        }

        let x = rng.range(1, MAP_WIDTH - 2);
        let y = rng.range(1, MAP_HEIGHT - 2);
        let pt = Point::new(x, y);
        if map.can_enter(pt) {
            ecs.push(stairs_down_tuple(pt, to_depth));
            return
        }
    }
}



