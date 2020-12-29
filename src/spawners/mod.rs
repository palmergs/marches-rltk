use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Player,
            Actor,
            pt,
            Render{ color: ColorPair::new(WHITE, BLACK), tile: 128 + 20 }
        )
    );
}