use crate::prelude::*;

mod rooms;
mod sewers;
mod empty;

// Number of rooms to attempt to place in a "standard" map
pub const NUM_ROOMS: usize = (
    (MAP_WIDTH as usize * MAP_HEIGHT as usize) / 
    (SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize)) * 20;

trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator, depth: i32) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub item_spawns: Vec<Point>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(depth: i32) -> Self {
        Self{
            map: Map::new(depth),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            item_spawns: Vec::new(),
            player_start: Point::zero(),
        }
    }

    pub fn build(rng: &mut Rng, depth: i32) -> MapBuilder {
        let mut architect: Box<dyn MapArchitect> = match depth {
            // 0     => Box::new(town::TownArchitect::new()),
            0     => Box::new(sewers::SewersArchitect::new()),
            _     => match rng.range(0, 10) {
                // 1 => Box::new(maze::MazeArchitect::new()),
                // 2 => Box::new(drunkard::DrunkardsWalkArchitect::new()),
                // 3 => Box::new(diffusion::DiffussionArchitect::new()),
                // 4 => Box::new(automota::CellularAutomotaArchitect::new()),
                // 5 => Box::new(voronoi::VoronoiHiveArchitect::new()),
                // 6 => Box::new(wave::WaveFunctionArchitect::new()),
                7 => Box::new(sewers::SewersArchitect::new()),
                8 => Box::new(empty::EmptyArchitect::new()),
                _ => Box::new(rooms::RoomsArchitect::new()),
            }
        };
        
        architect.build(rng, depth)
    }

    pub fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn excavate_random_rooms(&mut self, rng: &mut Rng, n: usize, floor: TileType) {
        let mut countdown = 1000;
        while self.rooms.len() < n {
            countdown -= 1;
            if countdown == 0 {
                println!("countdown expired in excavate random rooms!");
                return;
            }

            let rb = RoomBuilder::rect(rng, 3, 10);
            if !rb.intersect(&self.rooms) {
                rb.excavate(&mut self.map, floor);
                self.rooms.push(rb.extent);
            }
        }
    }

    fn excavate_tunnels(&mut self, rng: &mut Rng, floor: TileType) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let curr = room.center();
            self.excavate_tunnel(rng, prev, curr, floor);
        }
    }

    fn excavate_tunnel(&mut self, rng: &mut Rng, prev: Point, curr: Point, floor: TileType) {
        if rng.range(0,2) == 1 {
            self.excavate_horiz_tunnel(prev.x, curr.x, prev.y, floor);
            self.excavate_vert_tunnel(prev.y, curr.y, curr.x, floor);
        } else {
            self.excavate_vert_tunnel(prev.y, curr.y, prev.x, floor);
            self.excavate_horiz_tunnel(prev.x, curr.x, curr.y, floor);
        }
    }

    fn excavate_vert_tunnel(&mut self, y1: i32, y2: i32, x: i32, floor: TileType) {
        for y in std::cmp::min(y1, y2) ..= std::cmp::max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = floor;
            }
        }
    }

    fn excavate_horiz_tunnel(&mut self, x1: i32, x2: i32, y: i32, floor: TileType) {
        for x in std::cmp::min(x1, x2) ..= std::cmp::max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = floor;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoomShape {
    Rectangle,
    Rounded,
    RoughRectangle,
    RoughRounded,
    Diamond,
}

impl RoomShape {
    pub fn random(rng: &mut Rng) -> RoomShape {
        match rng.range(0, 4) {
            0 => RoomShape::Rounded,
            1 => RoomShape::RoughRectangle,
            2 => RoomShape::RoughRounded,
            3 => RoomShape::Diamond,
            _ => RoomShape::Rectangle,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoomBuilder {
    pub shape: RoomShape,
    pub extent: Rect,
}

impl RoomBuilder {
    pub fn new(shape: RoomShape, rng: &mut Rng, min: i32, max: i32) -> RoomBuilder {
        match shape {
            RoomShape::Rectangle => RoomBuilder::rect(rng, min, max),
            _ => RoomBuilder::rect(rng, min, max),
        }
    }

    pub fn rect(rng: &mut Rng, min: i32, max: i32) -> RoomBuilder {
        let (w, h) = if max <= min {
            (min, min)
        } else { 
            (rng.range(min, max + 1), rng.range(min, max + 1))
        };

        let extent = Rect::with_size(    
            rng.range(1, MAP_WIDTH as i32 - w - 1),
            rng.range(1, MAP_HEIGHT as i32 - h - 1),
            w,
            h);
        RoomBuilder{ shape: RoomShape::Rectangle, extent }
    }

    pub fn diamond(rng: &mut Rng, min: i32, max: i32) -> RoomBuilder {
        if min < 5 { return RoomBuilder::rect(rng, min, max); }

        let w = rng.range(min, max + 1);
        let h = rng.range(min, max + 1);
        let extent = Rect::with_size(    
            rng.range(1, MAP_WIDTH as i32 - w - 1),
            rng.range(1, MAP_HEIGHT as i32 - h - 1),
            w,
            h);
        RoomBuilder{ shape: RoomShape::Diamond, extent }
    }

    pub fn intersect(&self, rooms: &Vec<Rect>) -> bool {
        for r in rooms.iter() {
            if r.intersect(&self.extent) {
                return true;
            }
        }
        false
    }

    pub fn excavate(&self, map: &mut Map, floor: TileType) {
        match self.shape {
            RoomShape::Diamond => {
                let x1 = self.extent.x1;
                let x2 = self.extent.x2;
                let y1 = self.extent.y1;
                let y2 = self.extent.y2;
                let w = x2 - x2;
                let h2 = (y2 - y1) / 2;
                for y in 0 ..= h2 {
                    for x in 0 .. w {
                        // the number of spaces is the vertical % / 2
                        let ratio_h = (y + 1) / h2;
                        let spaces = w / 2 - ratio_h;
                        if x >= spaces && x < w - spaces {
                            let idx = map.point2d_to_index(Point::new(x1 + x, y1 + y));
                            map.tiles[idx] = floor;

                            let idx = map.point2d_to_index(Point::new(x1 + x, y2 - y));
                            map.tiles[idx] = floor;
                        }
                    }
                }
            },

            _ => {
                self.extent.for_each(|pt| {
                    if map.in_bounds(pt) {
                        let idx = map.point2d_to_index(pt);
                        map.tiles[idx] = floor
                    }
                });
            },
        }
    }
}