use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Physical)]
#[read_component(Mental)]
#[write_component(Stats)]
pub fn heal(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn: &TurnCount,
) {
    let mut rng = Rng::new();
    let mut query = <(&Point, &Physical, &mut Stats)>::query().filter(component::<Mental>());
    query.iter_mut(ecs)
        .for_each(|(pt, physical, stats)| {
            if stats.vigor.is_wounded() {
                let act_on = 100 - (physical.brawn.curr * 10) + rng.range(1, 10);
                if turn.act(act_on as usize) {
                    stats.vigor.curr += 1;
                    commands.push((Text{
                        display: TextDisplay::AnimateUp(*pt),
                        text: format!("{}", 1),
                        color: RGBA::named(PINK),
                        ticks: 50,
                        count: 0,
                    }, ));
                }
            }

            if stats.focus.is_wounded() {
                let act_on = 10 - (physical.brawn.curr) + rng.range(1, 10);
                if turn.act(act_on as usize) {
                    stats.focus.curr += 1;
                    commands.push((Text{
                        display: TextDisplay::AnimateUp(*pt),
                        text: format!("{}", 1),
                        color: RGBA::named(CYAN),
                        ticks: 50,
                        count: 0,
                    }, ));
                }
            }
        });
}
