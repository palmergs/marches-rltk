use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Actor;

#[derive(Debug, Clone, PartialEq)]
pub struct MightTalk {
    pub chance: i32, // in 1000
    pub phrase: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atts {
    pub curr: i32,
    pub max: i32,
}

impl Atts {
    pub fn new(n: i32) -> Self { Self{ curr: n, max: n } }
    pub fn heal(&self, n: i32) -> Self {
        let mut curr = self.curr + n;
        if curr > self.max { curr = self.max; }
        Self { curr, max: self.max }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attributes {
    pub brawn: Atts,
    pub grace: Atts,
    pub charm: Atts,
    pub smart: Atts,
    pub faith: Atts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Points {
    pub focus: Atts,
    pub vigor: Atts,
    pub karma: Atts,
    pub magic: Atts,
}