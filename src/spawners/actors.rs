use crate::prelude::*;

pub fn spawn_rat(ecs: &mut World, pt: Point) {
    ecs.push(
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
            MoveStrategy::Random(2),
            Outlook::Fearful,
        )
    );
}

pub fn spawn_bat(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Actor,
            Render{ 
                name: "Cave Bat".to_string(),
                tile: tile_index(12, 19),
                pt
            },
            FieldOfView::new(4),
            MoveStrategy::Random(1),
            Outlook::Neutral,
        )
    );
}

pub fn spawn_animated_tree(ecs: &mut World, pt: Point) {
    ecs.push(
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
            MoveStrategy::Random(10),
            Outlook::Neutral,
        )
    );
}

pub fn spawn_goblin_with_torch(ecs: &mut World, pt: Point) {
    ecs.push(
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
                    "Quiet you maggots! I heard something.".to_string(),
                    "This is our territory!".to_string(),
                ],
            },
            MoveStrategy::Patrol(1, Direction::random()),
            Outlook::Aggressive,
        )
    );
}

pub fn spawn_goblin(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Actor,
            Render{ 
                name: "Goblin".to_string(),
                tile: tile_index(12, 9),
                pt,
            },
            FieldOfView::new(7),
            FieldOfLight::new(5),
            MightTalk{
                chance: 5,
                phrases: vec![
                    "Meat's back on the menu!".to_string(),
                    "Wonder if this one will squeal?".to_string(),
                ],
            },
            MoveStrategy::Patrol(1, Direction::random()),
            Outlook::Aggressive,
        )
    );
}

pub fn spawn_skeleton_with_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Actor,
            Render{ 
                name: "Skeleton with torch".to_string(),
                tile: tile_index(12, 23),
                pt,
            },
            FieldOfView::new(5),
            MoveStrategy::Patrol(1, Direction::random()),
            Outlook::Aggressive,
        )
    );
}

pub fn spawn_skeleton(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Actor,
            Render{ 
                name: "Skeleton".to_string(),
                tile: tile_index(12, 23),
                pt,
            },
            FieldOfView::new(5),
            MoveStrategy::Patrol(1, Direction::random()),
            Outlook::Aggressive,
        )
    );
}