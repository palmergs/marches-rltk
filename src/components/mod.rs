pub use crate::prelude::*;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub tile: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name(String);

