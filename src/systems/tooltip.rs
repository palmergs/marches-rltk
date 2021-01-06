use crate::prelude::*;

use std::collections::HashSet;

#[system]
#[read_component(Render)]
#[read_component(Stats)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(Text)]
pub fn tooltip(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] camera: &Camera,
    #[resource] mouse: &Point,
) {
    let mut query = <&FieldOfView>::query().filter(component::<Player>());
    let fov = query.iter(ecs).next().unwrap();

    let mut query = <&Text>::query();
    let mut fades: HashSet<Point> = HashSet::new();
    query.iter(ecs)
        .filter(|text| text.is_fade())
        .for_each(|text| {
            fades.insert(text.pt());
        });

    let pointer = *mouse + camera.offset();

    let mut query = <(&Render, &Stats)>::query().filter(!component::<Player>());
    query.iter(ecs)
        .filter(|(render, _)| render.pt == pointer )
        .filter(|(render, _)| fov.visible_tiles.contains(&render.pt))
        .for_each(|(render, stats)| {
            if !fades.contains(&render.pt) {
                let (text, color) = if stats.vigor.is_wounded() {
                    (format!("{} (wounded)", render.name), RGBA::named(PINK))
                } else {
                    (format!("{}", render.name), RGBA::named(WHITE))
                };
                commands.push(((), Text{
                    display: TextDisplay::Fade(render.pt),
                    text,
                    color,
                    ticks: 40,
                    count: 0,
                }));
            }
        });
}
