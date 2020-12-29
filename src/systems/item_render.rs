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
    let mut renderables = <(&Point, &Render, &Item)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ITEM_LAYER);
    draw_batch.cls();

    let offset = camera.offset();
    renderables.iter(ecs)
        .for_each(|(pt, render, _)| {
            draw_batch.set(*pt - offset, render.color, render.tile);
        });

    draw_batch.set(Point::new(3, 10), ColorPair::new(WHITE, BLACK), 10);
    draw_batch.set(Point::new(3, 11), ColorPair::new(RED, BLACK), 10);
    draw_batch.set(Point::new(3, 12), ColorPair::new(BLUE, BLACK), 10);
    draw_batch.submit(0).expect("batch error in item_render");
}
