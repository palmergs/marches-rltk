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

    pub fn build(rng: &mut Rng, depth: i32) -> Self {
        let mut architect: Box<dyn MapArchitect> = match depth {
            0     => Box::new(town::TownArchitect::new()),
            1     => Box::new(sewers::SewerArchitect::new()),
            _     => match rng.range(0, 10) {
                1 => Box::new(maze::MazeArchitect::new()),
                2 => Box::new(drunkard::DrunkardsWalkArchitect::new()),
                3 => Box::new(diffusion::DiffussionArchitect::new()),
                4 => Box::new(automota::CellularAutomotaArchitect::new()),
                5 => Box::new(voronoi::VoronoiHiveArchitect::new()),
                6 => Box::new(wave::WaveFunctionArchitect::new()),
                _ => Box::new(rooms::RoomsArchitect::new()),
            }
        }
    }
}