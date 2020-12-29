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

    let offset = camera.offset();
    renderables
        .iter(ecs)
        .for_each(|(pt, render, _)| {
            // println!("actor at {:?} real={:?} tile={:?}", *pt - offset, *pt, render.tile);
            draw_batch.set(*pt - offset, render.color, render.tile);
        });

    draw_batch.submit(2000).expect("batch error in actor_render");
}
