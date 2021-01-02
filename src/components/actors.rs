#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Actor;

#[derive(Debug, Clone, PartialEq)]
pub struct MightTalk {
    pub chance: i32, // in 1000
    pub phrase: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atts(pub i32, pub i32);

impl Atts {
    pub fn new(n: i32) -> Self { Self(n, n) }
    pub fn heal(&mut self, n: i32)  {  self.1 = std::cmp::min(self.0, self.1 + n); }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attributes {
    pub brawn: Atts,
    pub grace: Atts,
    pub charm: Atts,
    pub smart: Atts,
    pub faith: Atts,
}

impl Attributes {
    pub fn new(b: i32, g: i32, c: i32, s: i32, f: i32) -> Attributes {
        Attributes{
            brawn: Atts::new(b),
            grace: Atts::new(g),
            charm: Atts::new(c),
            smart: Atts::new(s),
            faith: Atts::new(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Points {
    pub focus: Atts,
    pub vigor: Atts,
    pub karma: Atts,
    pub magic: Atts,
}

impl Points {
    pub fn new(f: i32, v: i32, k: i32, m: i32) -> Points {
        Points{
            focus: Atts::new(f),
            vigor: Atts::new(v),
            karma: Atts::new(k),
            magic: Atts::new(m),
        }
    }
}