use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MightTalk)]
pub fn might_talk(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    let mut rng = Rng::new();
    let color = RGBA::from_f32(1.0, 1.0, 0.0, 1.0);
    let ticks = 100;
    let count = 0;
    <(&Point, &MightTalk)>::query()
        .iter(ecs)
        .for_each(|(pt, talk)| {
            if talk.phrases.len() > 0 && rng.range(0, 1000) <= talk.chance {
                commands.push((Text{ 
                    display: TextDisplay::Fade(*pt + Point::new(0, -1)),
                    color,
                    text: talk.phrases[rng.range(0, talk.phrases.len())].clone(),
                    ticks,
                    count,
                }, ));
            }
        });
}