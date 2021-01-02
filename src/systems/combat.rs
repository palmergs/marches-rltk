use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Actor)]
#[read_component(Point)]
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
    if let Ok(mut pt) = ecs.entry_mut(cmd.victim).unwrap().get_component::<Point>() {
        let text = format!("{}", dmg).to_string();
        commands.push(((), FadingText{ pt: *pt, text, life: 100, remaining: 100 }));
    }
    commands.remove(*entity);
}