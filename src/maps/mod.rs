use crate::prelude::*;
use serde::{ Serialize, Deserialize };
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TileType {
    Floor(i32),
    Wall(i32),
}

impl TileType {
    pub fn stone() -> TileType { TileType::Wall(STONE) }
    pub fn floor() -> TileType { TileType::Wall(FLOOR) }
    pub fn water() -> TileType { TileType::Wall(WATER) }
    pub fn grass() -> TileType { TileType::Wall(GRASS) }
    pub fn tile_idx(&self) -> i32 {
        match self {
            TileType::Floor(n) => *n,
            TileType::Wall(n) => *n,
        }
    }
}

pub const GRASS:i32 = 2;
pub const WATER:i32 = 128 + 26;
pub const FLOOR:i32 = 128 + 18;
pub const STONE:i32 = 128 + 2;

// This is the current amount of navigable space at this tile 
// necessary to calculate passage for actors that take more than 
// one tile (max: 7)
// const SPACE:u32 =       0x000_0000;

// Up to 16 tile types are supported (which may look different depending on theme)
// 0.  0000 Void
// 1.  0001 Stone Floor 
// 2.  0010 Grass
// 3.  0011 Dirt
// 4.  0100 Wood Floor
// 5.  0101 Mud
// 6.  0110 Water Surface
// 7.  0111 Lava Surface
// 8.  1000 Solid Stone
// 9.  1001 Stonework (large stones)
// 10. 1010 Brick Wall (small rectangular stones)
// 11. 1011 Earth Wall (packed earth or sand)
// 12. 1100 Wood Wall (wood construction)
// 13. 1101 Deep Mud (character is swimming through mud)
// 14. 1110 Deep Water (character is swimming)
// 15. 1111 Deep Lava (character is swimming through lava)
//const TILE_TYPE:u32 =   0x0f00_0000;

// This tile is currently occupied by an actor
//const ACTOR:u32 =       0x0080_0000;

// This tile is currently blocked
//const BLOCKED:u32 =     0x0040_0000;

// This tile has been revealed to the player
//const REVEALED:u32 =    0x0020_0000;

// This tile has been visited by the player
//const VISITED:u32 =     0x0010_0000;

// This tile blocks light, heat and vision (max: 15)
//const OPACITY:u32 =     0x000f_0000;

// This tile is a source of heat (max: 15);
//const HSOURCE:u32 =     0x0000_f000;

// This tile is heated (which may occlude infravision) (max: 15)
//const HEAT:u32 =        0x0000_0f00;

// This tile is the source of light (max: 15)
//const LSOURCE:u32 =     0x0000_00f0;

// This tile is lit (max intensity: 15)
//const LIGHT:u32 =       0x0000_000f;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub depth: i32,
    pub origin: Point,
    pub extent: Point,
    pub tiles: Vec<TileType>,
    pub revealed: Vec<bool>,
    pub indoors: Vec<bool>,
    pub actors: HashSet<Point>,
    pub opaque: HashSet<Point>,
    pub blocked: HashSet<Point>,
}

pub const ORIGIN:Point = Point{ x: 0, y: 0 };
pub const EXTENT:Point = Point{ x: MAP_WIDTH as i32, y: MAP_HEIGHT as i32 };

impl Map {
    pub fn new(depth: i32) -> Self {
        Self{
            depth,
            origin: ORIGIN,
            extent: EXTENT,
            tiles: vec![TileType::Floor(FLOOR); MAP_TILES],
            revealed: vec![false; MAP_TILES],
            indoors: vec![true; MAP_TILES],
            actors: HashSet::new(),
            opaque: HashSet::new(),
            blocked: HashSet::new(),
        }
    }

    pub fn is_wall(&self, idx: usize) -> bool {
        match self.tiles[idx] {
            TileType::Wall(_) => true,
            _ => false,
        }
    }

    pub fn can_enter(&self, pt: Point) -> bool {
        if !self.in_bounds(pt) { return false; }
        if self.blocked.contains(&pt) { return false; }

        !self.is_wall(self.point2d_to_index(pt))
    }

    pub fn valid_exit(&self, pt: Point, delta: Point) -> Option<usize> {
        let check = pt + delta;
        if self.can_enter(check) {
            return Some(self.point2d_to_index(check))
        }
        None
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(self.point2d_to_index(point))
        } else {
            None
        }
    }

    #[inline]
    pub fn distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1),
            self.index_to_point2d(idx2))
    }

    pub fn is_passage(&self, pt: Point) -> bool {
        let c = self.can_enter(pt);
        let n = self.can_enter(pt + Point::new(  0, -1));
        let s = self.can_enter(pt + Point::new(  0,  1));
        let e = self.can_enter(pt + Point::new(  1,  0));
        let w = self.can_enter(pt + Point::new( -1,  0));
        (c && n && s && !e && !w) || (c && e && w && !n && !s)
    } 
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        self.extent
    }

    fn in_bounds(&self, pt: Point) -> bool {
        pt.x >= self.origin.x && pt.x < self.extent.x && pt.y >= self.origin.y && pt.y < self.extent.y
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let pt = self.index_to_point2d(idx);
        if let Some(idx) = self.valid_exit(pt, Point::new(-1,  0)) { exits.push((idx, 1.0)); }
        if let Some(idx) = self.valid_exit(pt, Point::new( 1,  0)) { exits.push((idx, 1.0)); }
        if let Some(idx) = self.valid_exit(pt, Point::new( 0, -1)) { exits.push((idx, 1.0)); }
        if let Some(idx) = self.valid_exit(pt, Point::new( 0,  1)) { exits.push((idx, 1.0)); }
        if let Some(idx) = self.valid_exit(pt, Point::new(-1, -1)) { exits.push((idx, 1.5)); }
        if let Some(idx) = self.valid_exit(pt, Point::new(-1,  1)) { exits.push((idx, 1.5)); }
        if let Some(idx) = self.valid_exit(pt, Point::new( 1, -1)) { exits.push((idx, 1.5)); }
        if let Some(idx) = self.valid_exit(pt, Point::new( 1,  1)) { exits.push((idx, 1.5)); }
        exits
    }

    fn is_opaque(&self, idx: usize) -> bool {
        if self.is_wall(idx) { return true; }

        let pt = self.index_to_point2d(idx);
        self.opaque.contains(&pt)
    }
}
