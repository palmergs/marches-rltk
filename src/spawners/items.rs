use crate::prelude::*;

pub fn torch_tuple(pt: Point) -> (Item, Point, Render, FieldOfLight, Stats) {
    (
        Item{
            blocking: false, 
            opaque: false,
            can_get: true,
        },
        pt,
        Render{
            name: "Torch".to_string(),
            tile: tile_index(1, 11),
        },
        FieldOfLight::new(5),
        Stats{
            armor: 0,
            speed: 0,
            power: 1,
            vigor: Vigor::new(3),
            focus: Focus::new(0),
        },
    )
}

pub fn dagger_tuple(pt: Point) -> (Item, Point, Render, Stats) {
    (
        Item{
            blocking: false, 
            opaque: false,
            can_get: true,
        },
        pt,
        Render{
            name: "Dagger".to_string(),
            tile: tile_index(13, 116),
        },
        Stats{
            armor: 0,
            speed: 0,
            power: 1,
            vigor: Vigor::new(10),
            focus: Focus::new(0),
        },
    )
}

pub fn mushroom1_tuple(pt: Point) -> (Item, Point, Render, Stats) {
        (
            Item{
                blocking: false, 
                opaque: false,
                can_get: true,
            },
            pt,
            Render{
                name: "Mushroom".to_string(),
                tile: tile_index(12, 35),
            },
            Stats{
                armor: 0,
                speed: 0,
                power: 0,
                vigor: Vigor::new(1),
                focus: Focus::new(0),
            },
        )
}

pub fn mushroom2_tuple(pt: Point) -> (Item, Point, Render, Stats) {
    (
        Item{
            blocking: false, 
            opaque: false,
            can_get: true,
        },
        pt,
        Render{
            name: "Mushroom".to_string(),
            tile: tile_index(13, 35),
        },
        Stats{
            armor: 0,
            speed: 0,
            power: 0,
            vigor: Vigor::new(1),
            focus: Focus::new(0),
        },
    )
}

pub fn chest_tuple(pt: Point) -> (Item, Point, Render, Stats, Spawns) {
    (
        Item{
            blocking: true,
            opaque: false,
            can_get: false,
        },
        pt,
        Render{
            name: "Closed Chest".to_string(),
            tile: tile_index(2, 7),
        },
        Stats{
            armor: 5,
            speed: 0,
            power: 0,
            vigor: Vigor::new(30),
            focus: Focus::new(0),
        },
        Spawns{
            entities: vec![ 
                SpawnEntity::new("skeleton", 100, SpawnTrigger::Killed),
                SpawnEntity::new("skeleton", 100, SpawnTrigger::Opened),
            ],
        },
    )
}

pub fn stairs_down_tuple(pt: Point, to_depth: i32) -> (Item, Point, Render, Stats, Stairs) {
    (
        Item{
            blocking: false, 
            opaque: false,
            can_get: false,
        },
        pt,
        Render{
            name: "Stairs Down".to_string(),
            tile: tile_index(2, 5),
        },
        Stats{
            armor: 5,
            speed: 0,
            power: 0,
            vigor: Vigor::new(30),
            focus: Focus::new(0),
        },        
        Stairs{ to_depth },
    )
}


pub fn stairs_up_tuple(pt: Point, to_depth: i32) -> (Item, Point, Render, Stats, Stairs) {
    (
        Item{
            blocking: false, 
            opaque: false,
            can_get: false,
        },
        pt,
        Render{
            name: "Stairs Up".to_string(),
            tile: tile_index(1, 5),
        },
        Stats{
            armor: 5,
            speed: 0,
            power: 0,
            vigor: Vigor::new(30),
            focus: Focus::new(0),
        },        
        Stairs{ to_depth },
    )
}

pub fn open_door_tuple(pt: Point) -> (Item, Point, Render, Stats) {
    (
        Item{
            blocking: false,
            opaque: false,
            can_get: false,
        },
        pt,
        Render{
            name: "Door".to_string(),
            tile: tile_index(1, 4),
        },
        Stats{
            armor: 3,
            speed: 0,
            power: 0,
            vigor: Vigor::new(30),
            focus: Focus::new(0),
        },
    )
}

pub fn closed_door_tuple(pt: Point) -> (Item, Point, Render, Stats, Spawns) {
    (
        Item{
            blocking: true, 
            opaque: true,
            can_get: false,
        },
        pt,
        Render{
            name: "Door".to_string(),
            tile: tile_index(1, 3),
        },
        Stats{
            armor: 3,
            speed: 0,
            power: 0,
            vigor: Vigor::new(30),
            focus: Focus::new(0),
        },
        Spawns{
            entities: vec![ SpawnEntity::new("doormouse", 200i32, SpawnTrigger::Killed) ],
        },
    )
}
