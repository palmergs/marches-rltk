use crate::prelude::*;

use super::MapArchitect;

pub struct SewersArchitect;

impl SewersArchitect {
    pub fn new() -> Self {
        Self{}
    }
}

impl MapArchitect for SewersArchitect {
    fn build(&mut self, rng: &mut Rng, depth: i32) -> MapBuilder {
        let mut mb = MapBuilder::new(depth);
        mb.fill(TileType::Wall(STONE));

        let grid = 9;
        let gridh = MAP_HEIGHT / grid;
        let gridw = MAP_WIDTH / grid;
        for r in 0..grid {
            for c in 0..grid {
                let w = rng.range(gridw / 2, gridw);
                let h = rng.range(gridh / 3, gridh - 2);
                let x = (c * gridw) + (gridw - w) / 2;
                let y = (r * gridh) + (gridh - h) / 2;
                let rb = RoomBuilder{ extent: Rect::with_size(x, y, w, h), shape: RoomShape::Rectangle };
                rb.excavate(&mut mb.map, TileType::Floor(FLOOR));
                mb.rooms.push(rb.extent);
            }
        }

        // make sure each room is attached; not using the
        // default tunnel builder because I don't want to sort them
        let rooms = mb.rooms.clone();
        for i in 1 .. rooms.len() {
            let prev = rooms[i-1].center();
            let curr = rooms[i].center();
            mb.excavate_tunnel(rng, prev, curr, TileType::Floor(WATER));
            if i >= grid && rng.range(0, 2) == 0 {
                let prev = rooms[i-grid].center();
                let curr = rooms[i].center();
                mb.excavate_tunnel(rng, prev, curr, TileType::Floor(WATER));
            }
        }

        mb.player_start = mb.rooms[0].center();
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        mb
    }
}