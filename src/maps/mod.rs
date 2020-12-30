use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Tree,
    Door,
    DoorOpen,
    Bookshelf,
    Chest,
    ChestEmpty,
}

pub struct Map {
    pub depth: i32,
    pub origin: Point,
    pub extent: Point,
    pub tiles: Vec<TileType>,
    pub revealed: Vec<bool>
}

impl Map {
    pub fn new() -> Self {
        let mut tiles = vec![TileType::Floor; MAP_TILES];
        let mut rng = Rng::new();
        for _ in 0 .. 2000 {
            match rng.range(0, 4) {
                0 => tiles[rng.range(0, MAP_TILES)] = TileType::Wall,
                1 => tiles[rng.range(0, MAP_TILES)] = TileType::Door,
                2 => tiles[rng.range(0, MAP_TILES)] = TileType::Tree,
                _ => tiles[rng.range(0, MAP_TILES)] = TileType::Wall,
            }
        }
        Self{
            depth: 0,
            origin: Point::constant(0, 0),
            extent: Point::constant(MAP_WIDTH as i32, MAP_HEIGHT as i32),
            tiles,
            revealed: vec![false; MAP_TILES],
        }
    }

    pub fn in_bounds(&self, pt: Point) -> bool {
        pt.x >= 0 && pt.x < self.extent.x && pt.y >= 0 && pt.y < self.extent.y
    }

    pub fn can_enter(&self, pt: Point) -> bool {
        if !self.in_bounds(pt) { return false; }

        let idx = self.point2d_to_index(pt);
        let tile = self.tiles[idx];
        tile == TileType::Floor || tile == TileType::DoorOpen
    }

    pub fn valid_exit(&self, pt: Point, delta: Point) -> Option<usize> {
        let check = pt + delta;
        if self.can_enter(check) {
            return Some(self.point2d_to_index(check))
        }
        None
    }

    pub fn distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1),
            self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        self.extent
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
        let tile = self.tiles[idx];
        tile == TileType::Wall
            || tile == TileType::Tree
            || tile == TileType::Door
            || tile == TileType::Bookshelf
    }
}