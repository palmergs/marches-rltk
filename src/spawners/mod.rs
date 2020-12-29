use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            pt,
            Render{ color: ColorPair::new(WHITE, BLACK), tile: 7 }
        )
    );
}