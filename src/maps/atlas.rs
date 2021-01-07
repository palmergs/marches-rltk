use crate::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtlasPoint {
    pub top: i32,
    pub left: i32,
    pub depth: i32,
}

impl AtlasPoint {
    pub new(pt: Point, depth: i32) -> Self {
        AtlasPoint{ top: pt.y, left: pt.x, depth }
    }
}

#[derive(Debug)]
pub struct MapResource {
    map: &Map,
}

#[derive(Debug)]
pub struct Atlas {
    pub maps: HashMap<AtlasPoint, &MapResource>,
}

impl Default for Atlas {
    fn default() -> Self {
        Atlas{ maps: HashMap::new() }
    }
}

impl Atlas {
    pub fn build_map(&mut self, apt: AtlasPoint) -> Map {
        let mut rng = Rng::new();
        let mb = MapBuilder::build(&mut rng, apt.depth);
        mb
    }
}

