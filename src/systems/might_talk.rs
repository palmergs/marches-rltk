use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MightTalk)]
pub fn might_talk(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    let mut rng = Rng::new();
    let life = 100;
    let remaining = 100;
    <(&Point, &MightTalk)>::query()
        .iter(ecs)
        .for_each(|(pt, talk)| {
            if rng.range(0, 1000) <= talk.chance {
                let pt = *pt + Point::new(0, -1);
                let text = talk.phrase.clone();
                commands.push(((), FadingText{ pt, text, life, remaining }));
            }
        });
}