use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Actor)]
#[read_component(Render)]
pub fn combat(
    entity: &Entity,
    cmd: &WantsToAttack,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    #[resource] _tick: &TickCount,
) {
    let mut rng = Rng::new();
    let dmg = rng.range(1, 5);

    if let Ok(render) = ecs.entry_ref(cmd.victim).unwrap().get_component::<Render>() {
        let text = format!("{}", dmg).to_string();
        commands.push(((), Text{ 
            display: TextDisplay::AnimateUp(render.pt),
            text,
            color: RGBA::from_f32(1., 0., 0., 1.),
            ticks: 100,
            count: 0,
        }));
    }


    commands.remove(*entity);
}