use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Actor)]
#[read_component(Point)]
#[write_component(Points)]
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

    let mut msg:Vec<String> = vec![];

    if let Ok((pt)) = ecs.entry_ref(cmd.victim).unwrap().get_component::<(Point)>() {
        let text = format!("{}", dmg).to_string();
        commands.push(((), FadingUpText{ pt: *pt, text, life: 100, remaining: 100 }));
    }


    commands.remove(*entity);
}