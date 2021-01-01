use crate::prelude::*;

use super::MapArchitect;

pub const NUM_ROOMS: usize = (
    (MAP_WIDTH as usize * MAP_HEIGHT as usize) / 
    (SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize)) * 20;

pub struct RoomsArchitect;

impl RoomsArchitect {
    pub fn new() -> Self {
        Self{}
    }
}

impl MapArchitect for RoomsArchitect {
    fn build(&mut self, rng: &mut Rng, depth: i32) -> MapBuilder {
        let mut mb = MapBuilder::new(depth);
        mb.fill(TileType::Wall);
        mb.excavate_random_rooms(rng, NUM_ROOMS, TileType::Floor);
        mb.excavate_tunnels(rng, TileType::Floor);

        mb.player_start = mb.rooms[0].center();
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        
        mb
    }
}