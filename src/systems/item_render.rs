use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Item)]
#[read_component(Player)]
pub fn item_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera
) {
    let mut renderables = <(&Point, &Render)>::query().filter(component::<Item>());

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ITEM_LAYER);

    let offset = camera.offset();
    renderables.iter(ecs)
        .for_each(|(pt, render)| {
            draw_batch.set(*pt - offset, render.color, render.tile);
        });
    draw_batch.submit(5000).expect("batch error in item_render")
}
