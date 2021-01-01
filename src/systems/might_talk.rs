use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MightTalk)]
pub fn might_talk(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    let mut rng = Rng::new();
    <(&Point, &MightTalk)>::query()
        .iter(ecs)
        .for_each(|(pt, talk)| {
            if rng.range(0, 1000) <= talk.chance {
                println!("want to talk");
                commands.push(((), FadingText{ 
                    pt: *pt, 
                    text: talk.phrase.clone(), 
                    life: 100, 
                    remaining: 100 }));
            }
        });
}