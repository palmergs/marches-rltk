use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Actor)]
#[read_component(Player)]
pub fn actor_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera
) {
    let mut renderables = <(&Point, &Render, &Actor)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ACTOR_LAYER);
    draw_batch.cls();

    let offset = camera.offset();
    renderables
        .iter(ecs)
        .for_each(|(pt, render, _)| {
            // println!("actor at {:?} real={:?} tile={:?}", *pt - offset, *pt, render.tile);
            draw_batch.set(*pt - offset, render.color, render.tile);
        });

    draw_batch.set(Point::new(10, 10), ColorPair::new(WHITE, BLACK), 148);
    draw_batch.set(Point::new(11, 11), ColorPair::new(RED, BLACK), 148);
    draw_batch.set(Point::new(12, 12), ColorPair::new(BLUE, BLACK), 148);
    draw_batch.submit(0).expect("batch error in actor_render");
}
