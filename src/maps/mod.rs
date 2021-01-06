use crate::prelude::*;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

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

impl Map {
    pub fn new(depth: i32) -> Self {
        Self{
            depth,
            origin: Point::constant(0, 0),
            extent: Point::constant(MAP_WIDTH as i32, MAP_HEIGHT as i32),
            tiles: vec![TileType::Floor; MAP_TILES],
            revealed: vec![false; MAP_TILES],
            indoors: vec![true; MAP_TILES],
            actors: HashSet::new(),
            opaque: HashSet::new(),
            blocked: HashSet::new(),
        }
    }

    pub fn can_enter(&self, pt: Point) -> bool {
        if !self.in_bounds(pt) { return false; }
        if self.blocked.contains(&pt) { return false; }

        let idx = self.point2d_to_index(pt);
        self.tiles[idx] == TileType::Floor
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

    #[inline]
    pub fn font_idx(&self, idx: usize) -> usize {
        match self.tiles[idx] {
            TileType::Floor =>  tile_index(2, 19),
            TileType::Wall =>   tile_index(2, 3),
        }
    }

    pub fn is_passage(&self, pt: Point) -> bool {
        let n = self.can_enter(pt + Point::new(  0, -1));
        let s = self.can_enter(pt + Point::new(  0,  1));
        let e = self.can_enter(pt + Point::new(  1,  0));
        let w = self.can_enter(pt + Point::new( -1,  0));
        (n && s && !e && !w) || (e && w && !n && !s)
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
        if self.tiles[idx] == TileType::Wall { return true; }

        let pt = self.index_to_point2d(idx);
        self.opaque.contains(&pt)
    }
}