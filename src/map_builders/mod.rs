use crate::prelude::*;

// Number of rooms to attempt to place in a "standard" map
pub const NUM_ROOMS: usize = (
    (MAP_WIDTH as usize * MAP_HEIGHT as usize) / 
    (SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize)) * 20;

trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new() -> Self {
        Self{
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point,
        }
    }
}