use crate::prelude::*;

#[system(for_each)]
pub fn fading_text(
    entity: &Entity,
    cmd: &mut FadingText,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] tick: &TickCount,
    #[resource] camera: &Camera,
) {
    if cmd.remaining > 1 {
        let fg = cmd.remaining as f32 / cmd.life as f32;
        cmd.remaining -= 1;

        println!("fading text: {:?}", cmd);

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(UI_LAYER);
        draw_batch.print_color(
            cmd.pt - camera.offset(),
            cmd.text.clone(),
            ColorPair::new(RGBA::from_f32(1.0, 1.0, 0.0, fg), BLACK));
        draw_batch.submit(4000).expect("error rendering fading text");
    } else {
        println!("fading text removed: {:?}", cmd);
        commands.remove(*entity);
    }
}