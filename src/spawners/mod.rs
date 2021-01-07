use crate::prelude::*;

mod items;
pub use items::*;

mod actors;
pub use actors::*;

mod areas;
pub use areas::*;

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

pub fn spawn(id: &str, ecs: &mut World, pt: Point) -> Option<Entity> {
    let entity = match id {
        "torch" =>          ecs.push(torch_tuple(pt)),
        "dagger" =>         ecs.push(dagger_tuple(pt)),
        "open door" =>      ecs.push(open_door_tuple(pt)),
        "closed door" =>    ecs.push(closed_door_tuple(pt)),
        "chest" =>          ecs.push(chest_tuple(pt)),
        "mushroom1" =>      ecs.push(mushroom1_tuple(pt)),
        "mushroom2" =>      ecs.push(mushroom2_tuple(pt)),

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
    match rng.range(0 + depth, 14 + depth) {
        0..=3  => spawn("dagger", ecs, pt),
        4      => spawn("chest", ecs, pt),
        10     => spawn("mushroom1", ecs, pt),
        11     => spawn("mushroom2", ecs, pt),
        _      => None
    }
}

