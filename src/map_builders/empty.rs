use crate::prelude::*;

use super::MapArchitect;

pub struct EmptyArchitect;

impl EmptyArchitect {
    pub fn new() -> Self {
        Self{}
    }
}

impl MapArchitect for EmptyArchitect {
    fn build(&mut self, rng: &mut Rng, depth: i32) -> MapBuilder {
        let mut mb = MapBuilder::new(depth);
        mb.fill(TileType::Wall(STONE));
        for y in 1 .. MAP_HEIGHT - 1 {
            for x in 1 .. MAP_WIDTH - 1 {
                let idx = mb.map.point2d_to_index(Point::new(x, y));
                mb.map.tiles[idx] = TileType::Floor(FLOOR);
            }
        }

        mb.player_start = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        for _ in 0..100 {
            let pt = Point::new(
                rng.range(1, MAP_WIDTH - 1), 
                rng.range(1, MAP_HEIGHT - 1));
            mb.monster_spawns.push(pt) 
        }
        mb
    }
}