use crate::prelude::*;

use std::collections::HashSet;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Stats)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Text)]
pub fn tooltip(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] mouse: &Point,
) {
    let mut query = <&FieldOfView>::query().filter(component::<Player>());
    let fov = query.iter(ecs).next().unwrap();

    let pointer = *mouse + camera.offset();
    let mut query = <(&Point, &Render, &Stats)>::query().filter(!component::<Player>());
    query.iter(ecs)
        .filter(|(pt, _, _)| **pt == pointer )
        .filter(|(pt, _, _)| fov.visible_tiles.contains(pt))
        .for_each(|(pt, render, stats)| {
            let (text, color) = if stats.vigor.is_wounded() {
                (format!("{} (wounded)", render.name), ColorPair::new(RGBA::named(PINK), BLACK))
            } else {
                (format!("{}", render.name), ColorPair::new(RGBA::named(WHITE), BLACK))
            };
            let mut draw_batch = DrawBatch::new();
            draw_batch.target(UI_LAYER);
            draw_batch.print_color((*pt - camera.offset()) * 2, text, color);
            draw_batch.submit(5000).expect("error rendering tooltip text");
        });
}
