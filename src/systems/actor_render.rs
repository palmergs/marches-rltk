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
    let mut renderables = <(&Point, &Render)>::query().filter(component::<Actor>());

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ACTOR_LAYER);

    let offset = camera.offset();
    renderables.iter(ecs)
        .for_each(|(pt, render)| {
            draw_batch.set(*pt - offset, render.color, render.tile);
        });
    draw_batch.submit(5000).expect("batch error in actor_render")
}
