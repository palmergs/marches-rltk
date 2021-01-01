use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Actor;

#[derive(Debug, Clone, PartialEq)]
pub struct MightTalk {
    pub chance: i32, // in 1000
    pub phrase: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub curr: i32,
    pub max: i32,
}

impl Attribute {
    pub fn new(n: i32) -> Self { Self{ curr: n, max: n } }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attributes {
    pub brawn: Attribute,
    pub grace: Attribute,
    pub charm: Attribute,
    pub smart: Attribute,
    pub faith: Attribute,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Points {
    pub focus: Attribute,
    pub vigor: Attribute,
    pub karma: Attribute,
    pub magic: Attribute,
}