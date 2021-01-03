use crate::prelude::*;

// Physical attributes associated with an entity that can engage in
// its environment. Actors yes, items no.
#[derive(Debug, Clone, PartialEq)]
pub struct Physical {
    pub brawn: Brawn,
    pub grace: Grace,
}

impl Physical {
    pub fn new(brawn: i32, grace: i32) -> Self {
        Self{
            brawn: Brawn::new(brawn),
            grace: Grace::new(grace),
        }
    }
}

// Mental attributes associated with an entity that can engage in at least
// rudamentary thought; actors yes, items no (except possible magic items)
#[derive(Debug, Clone, PartialEq)]
pub struct Mental {
    pub outlook: Outlook,
    pub strategy: MoveStrategy,

    pub charm: Charm,
    pub smart: Smart,
}

impl Mental {
    pub fn new(outlook: Outlook, strategy: MoveStrategy, charm: i32, smart: i32) -> Self {
        Self{
            outlook,
            strategy,
            charm: Charm::new(charm),
            smart: Smart::new(smart),
        }
    }

    pub fn new_strategy(&self, can_see: bool) -> Option<MoveStrategy> {
        match self.outlook {
            Outlook::Aggressive => {
                match self.strategy {
                    MoveStrategy::Chase => {
                        if !can_see {
                            if self.smart.curr >= 0 {
                                return Some(MoveStrategy::Patrol(Direction::random()));
                            } else {
                                return Some(MoveStrategy::Random);
                            }
                        }
                    },
                    _ => {
                        if can_see {
                            return Some(MoveStrategy::Chase);
                        }
                    }
                }
            },
            Outlook::Fearful => {
                match self.strategy {
                    MoveStrategy::Flee => {
                        if !can_see {
                            if self.smart.curr >= 0 {
                                return Some(MoveStrategy::Patrol(Direction::random()));
                            } else {
                                return Some(MoveStrategy::Random);
                            }
                        }
                    },
                    _ => if can_see { return Some(MoveStrategy::Flee); }
                }
            },
            _ => ()
        }

        None
    }
}

// Attributes associated with an entity that can be damaged and and moved.
// Actors ues, items yes.
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub speed: usize,
    pub armor: i32,
    pub vigor: Vigor,
    pub focus: Focus,
}

impl Stats {
    pub fn new(armor: i32, speed: usize, vigor: i32, focus: i32) -> Self {
        Self {
            armor,
            speed,
            vigor: Vigor::new(vigor),
            focus: Focus::new(focus),
        }
    }
}

pub trait ValueWithMax {
    fn new(n: i32) -> Self;

    fn max(&self) -> i32;

    fn curr(&self) -> i32;

    fn is_zero(&self) -> bool { self.curr() <= 0 }

    fn is_max(&self) -> bool { self.max() == self.curr() }

    fn is_wounded(&self) -> bool { self.curr() < self.max() }

    fn is_bonus(&self) -> bool { self.curr() > 0 }

    fn is_penalty(&self) -> bool { self.curr() < 0 }

    fn heal(&self, n: i32) -> i32 { std::cmp::min(self.max() - self.curr(), n) }

    fn hit(&self, n: i32) -> (i32, i32) {
        if n <= self.curr() { return (n, 0); }
        return (self.curr(), n - self.curr());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Brawn {
    pub curr: i32,
    pub max: i32
}

impl Brawn {
    pub fn new(n: i32) -> Self { Self{ curr: n, max: n } }
}

impl ValueWithMax for Brawn {
    fn new(n: i32) -> Self { Self{ curr: n, max: n } }

    #[inline]
    fn max(&self) -> i32 { self.max }

    #[inline]
    fn curr(&self) -> i32 { self.curr }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grace {
    pub curr: i32,
    pub max: i32,
}

impl Grace {
    pub fn new(n: i32) -> Self { Self{ curr: n, max: n } }
}

impl ValueWithMax for Grace {
    fn new(n: i32) -> Self { Self{ curr: n, max: n } }

    #[inline]
    fn max(&self) -> i32 { self.max }

    #[inline]
    fn curr(&self) -> i32 { self.curr }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Charm {
    pub curr: i32,
    pub max: i32,
}

impl Charm {
    pub fn new(n: i32) -> Self { Self{ curr: n, max: n } }
}

impl ValueWithMax for Charm {
    fn new(n: i32) -> Self { Self{ curr: n, max: n } }

    #[inline]
    fn max(&self) -> i32 { self.max }

    #[inline]
    fn curr(&self) -> i32 { self.curr }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Smart {
    pub curr: i32,
    pub max: i32,
}

impl Smart {
    pub fn new(n: i32) -> Self { Self{ curr: n, max: n } }
}

impl ValueWithMax for Smart {
    fn new(n: i32) -> Self { Self{ curr: n, max: n } }

    #[inline]
    fn max(&self) -> i32 { self.max }

    #[inline]
    fn curr(&self) -> i32 { self.curr }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Focus {
    pub curr: i32,
    pub max: i32,
}

impl Focus {
    pub fn new(n: i32) -> Self { Self{ curr: n, max: n } }
}

impl ValueWithMax for Focus {
    fn new(n: i32) -> Self { Self{ curr: n, max: n } }

    #[inline]
    fn max(&self) -> i32 { self.max }

    #[inline]
    fn curr(&self) -> i32 { self.curr }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vigor {
    pub curr: i32,
    pub max: i32,
}

impl Vigor {
    pub fn new(n: i32) -> Self { Self{ curr: n, max: n } }
}

impl ValueWithMax for Vigor {
    fn new(n: i32) -> Self { Self{ curr: n, max: n } }

    #[inline]
    fn max(&self) -> i32 { self.max }

    #[inline]
    fn curr(&self) -> i32 { self.curr }
}
