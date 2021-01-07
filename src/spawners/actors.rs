use crate::prelude::*;

pub fn rat_tuple(pt: Point) -> (Actor, Render, FieldOfView, MightTalk, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Dungeon Rat".to_string(),
            tile: tile_index(13, 1),
            pt
        },
        FieldOfView::new(4),
        MightTalk{
            chance: 20,
            phrases: vec!["squeek!".to_string()]
        },
        Stats {
            armor: 0,
            speed: 2,
            vigor: Vigor::new(5),
            focus: Focus::new(4),
        },
        Physical{
            brawn: Brawn::new(-2),
            grace: Grace::new(2),
        },
        Mental{
            outlook: Outlook::Fearful,
            strategy: MoveStrategy::Random,
            smart: Smart::new(-3),
            charm: Charm::new(-1),
        }
    )
}

pub fn giant_rat_tuple(pt: Point) -> (Actor, Render, FieldOfView, MightTalk, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Giant Rat".to_string(),
            tile: tile_index(13, 2),
            pt
        },
        FieldOfView::new(4),
        MightTalk{
            chance: 20,
            phrases: vec!["squeek!".to_string()]
        },
        Stats {
            armor: 0,
            speed: 2,
            vigor: Vigor::new(8),
            focus: Focus::new(5),
        },
        Physical{
            brawn: Brawn::new(-1),
            grace: Grace::new(2),
        },
        Mental{
            outlook: Outlook::Aggressive,
            strategy: MoveStrategy::Random,
            smart: Smart::new(-2),
            charm: Charm::new(-1),
        }
    )
}

pub fn doormouse_tuple(pt: Point) -> (Actor, Render, FieldOfView, MightTalk, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Dire Dormouse".to_string(),
            tile: tile_index(13, 6),
            pt
        },
        FieldOfView::new(4),
        MightTalk{
            chance: 20,
            phrases: vec!["Squeek!!".to_string()]
        },
        Stats {
            armor: 0,
            speed: 2,
            vigor: Vigor::new(8),
            focus: Focus::new(5),
        },
        Physical{
            brawn: Brawn::new(-1),
            grace: Grace::new(2),
        },
        Mental{
            outlook: Outlook::Aggressive,
            strategy: MoveStrategy::Random,
            smart: Smart::new(-2),
            charm: Charm::new(-1),
        }
    )
}

pub fn bat_tuple(pt: Point) -> (Actor, Render, FieldOfView, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Cave Bat".to_string(),
            tile: tile_index(12, 19),
            pt
        },
        FieldOfView::new(4),
        Stats {
            armor: 0,
            speed: 1,
            vigor: Vigor::new(5),
            focus: Focus::new(4),
        },
        Physical{
            brawn: Brawn::new(-2),
            grace: Grace::new(2),
        },
        Mental{
            outlook: Outlook::Neutral,
            strategy: MoveStrategy::Random,
            smart: Smart::new(-3),
            charm: Charm::new(-1),
        }
    )
}

pub fn animated_tree_tuple(pt: Point) -> (Actor, Render, FieldOfView, MightTalk, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Animated Tree".to_string(),
            tile: tile_index(1, 22),
            pt,
        },
        FieldOfView::new(4),
        MightTalk{
            chance: 1,
            phrases: vec!["Haroom!".to_string()],
        },
        Stats {
            armor: 3,
            speed: 5,
            vigor: Vigor::new(40),
            focus: Focus::new(10),
        },
        Physical{
            brawn: Brawn::new(4),
            grace: Grace::new(-3),
        },
        Mental{
            outlook: Outlook::Neutral,
            strategy: MoveStrategy::Random,
            smart: Smart::new(0),
            charm: Charm::new(0),
        }
    )
}

pub fn goblin_with_torch_tuple(pt: Point) -> (Actor, Render, FieldOfView, FieldOfLight, MightTalk, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Goblin with torch".to_string(),
            tile: tile_index(12, 9),
            pt,
        },
        FieldOfView::new(7),
        FieldOfLight::new(5),
        MightTalk{
            chance: 5,
            phrases: vec![
                "Quiet you maggots!".to_string(),
                "Over there! Get it!".to_string(),
                "I hate this job".to_string(),
                "This is our territory!".to_string(),
                "I got the torch, I'm in charge".to_string(),
            ],
        },
        Stats {
            armor: 0,
            speed: 2,
            vigor: Vigor::new(10),
            focus: Focus::new(5),
        },
        Physical{
            brawn: Brawn::new(0),
            grace: Grace::new(1),
        },
        Mental{
            outlook: Outlook::Aggressive,
            strategy: MoveStrategy::Patrol(Direction::random()),
            smart: Smart::new(-1),
            charm: Charm::new(-2),
        }
    )
}

pub fn goblin_tuple(pt: Point) -> (Actor, Render, FieldOfView, MightTalk, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Goblin".to_string(),
            tile: tile_index(12, 10),
            pt,
        },
        FieldOfView::new(7),
        MightTalk{
            chance: 5,
            phrases: vec![
                "Meat's back on the menu!".to_string(),
                "Wonder if it will squeal?".to_string(),
                "I heard something!".to_string(),
            ],
        },
        Stats {
            armor: 0,
            speed: 2,
            vigor: Vigor::new(10),
            focus: Focus::new(5),
        },
        Physical{
            brawn: Brawn::new(0),
            grace: Grace::new(1),
        },
        Mental{
            outlook: Outlook::Aggressive,
            strategy: MoveStrategy::Patrol(Direction::random()),
            smart: Smart::new(-1),
            charm: Charm::new(-2),
        }
    )
}

pub fn skeleton_with_torch_tuple(pt: Point) -> (Actor, Render, FieldOfView, FieldOfLight, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Skeleton with torch".to_string(),
            tile: tile_index(12, 24),
            pt,
        },
        FieldOfView::new(5),
        FieldOfLight::new(5),
        Stats {
            armor: 1,
            speed: 2,
            vigor: Vigor::new(15),
            focus: Focus::new(0),
        },
        Physical{
            brawn: Brawn::new(0),
            grace: Grace::new(-1),
        },
        Mental{
            outlook: Outlook::Aggressive,
            strategy: MoveStrategy::Random,
            smart: Smart::new(-3),
            charm: Charm::new(-3),
        }
    )
}

pub fn skeleton_tuple(pt: Point) -> (Actor, Render, FieldOfView, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Skeleton".to_string(),
            tile: tile_index(12, 23),
            pt,
        },
        FieldOfView::new(5),
        Stats {
            armor: 1,
            speed: 2,
            vigor: Vigor::new(15),
            focus: Focus::new(0),
        },
        Physical{
            brawn: Brawn::new(0),
            grace: Grace::new(-1),
        },
        Mental{
            outlook: Outlook::Aggressive,
            strategy: MoveStrategy::Random,
            smart: Smart::new(-3),
            charm: Charm::new(-3),
        }
    )
}

pub fn skeleton_warrior_tuple(pt: Point) -> (Actor, Render, FieldOfView, Stats, Physical, Mental) {
    (
        Actor,
        Render{
            name: "Skeleton Warrior".to_string(),
            tile: tile_index(12, 26),
            pt,
        },
        FieldOfView::new(5),
        Stats {
            armor: 1,
            speed: 2,
            vigor: Vigor::new(15),
            focus: Focus::new(0),
        },
        Physical{
            brawn: Brawn::new(0),
            grace: Grace::new(-1),
        },
        Mental{
            outlook: Outlook::Aggressive,
            strategy: MoveStrategy::Random,
            smart: Smart::new(-3),
            charm: Charm::new(-3),
        }
    )
}