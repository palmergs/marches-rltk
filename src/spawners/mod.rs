use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Player,
            Actor,
            pt,
            Render{ color: ColorPair::new(WHITE, BLACK), tile: 128 + 20 },
            FieldOfView::new(10),
            FieldOfLight::new(5),
        )
    );
}

pub fn spawn_torch(ecs: &mut World, pt: Point) {
    ecs.push(
        (
            Item,
            pt,
            Render{ color: ColorPair::new(WHITE, BLACK), tile: 10 },
            FieldOfLight::new(5_,)
        )
    );
}