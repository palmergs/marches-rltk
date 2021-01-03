use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Render)]
#[write_component(Stats)]
pub fn combat(
    entity: &Entity,
    cmd: &WantsToAttack,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
 ) {
    let mut rng = Rng::new();
    let dmg = rng.range(1, 5);

    if let Ok(mut stats) = ecs.entry_mut(cmd.victim).unwrap().get_component_mut::<Stats>() {
        let (focus_dmg, vigor_dmg) = stats.focus.hit(dmg);
        let focus = stats.focus.curr - focus_dmg;
        let vigor = stats.vigor.curr - vigor_dmg;
        stats.focus.curr = focus;
        stats.vigor.curr = vigor;
        let killed = stats.vigor.curr <= 0;
        if let Ok(render) = ecs.entry_ref(cmd.victim).unwrap().get_component::<Render>() {
            if killed {
                let text = format!("KILLED!").to_string();
                commands.push(((), Text{
                    display: TextDisplay::AnimateUp(render.pt),
                    text,
                    color: RGBA::from_f32(1., 0., 0., 1.),
                    ticks: 200,
                    count: 0,
                }));

                // TODO: for ease of access to pt, removing the entity here, but this
                // might cause problems in the future if an entity with Stats but not Render
                // is killed.
                commands.remove(cmd.victim);
                map.actors.remove(&render.pt);
            } else {
                println!("remaining: focus={:?} vigor={:?}", focus, vigor);
                let text = format!("{}", dmg).to_string();
                commands.push(((), Text{
                    display: TextDisplay::AnimateUp(render.pt),
                    text,
                    color: RGBA::from_f32(1., 0., 0., 1.),
                    ticks: 50,
                    count: 0,
                }));
            }
        }
    }

    commands.remove(*entity);
}