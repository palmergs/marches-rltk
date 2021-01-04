use crate::prelude::*;

#[system]
#[read_component(Render)]
#[read_component(Stats)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn mouse(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] camera: &Camera,
    #[resource] mouse: &Point,
) {
    let mut query = <&FieldOfView>::query().filter(component::<Player>());
    let fov = query.iter(ecs).next().unwrap();

    let pointer = *mouse + camera.offset();

    let mut query = <(Entity, &Render, &Stats)>::query().filter(!component::<Player>());
    query.iter(ecs)
        .filter(|(_, render, _)| render.pt == pointer )
        .filter(|(_, render, _)| fov.visible_tiles.contains(&render.pt))
        .for_each(|(entity, render, stats)| {
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
        });
}