use crate::prelude::*;

#[system(for_each)]
pub fn fading_up_text(
    entity: &Entity,
    cmd: &mut FadingUpText,
    commands: &mut CommandBuffer,
    #[resource] camera: &Camera,
) {
    if cmd.remaining > 1 {
        let fg = cmd.remaining as f32 / cmd.life as f32;
        let offset = cmd.life / 4;
        let offset = 4 - (cmd.remaining / offset);

        cmd.remaining -= 1;        
        if camera.in_central_view(cmd.pt) {
            let mut draw_batch = DrawBatch::new();
            draw_batch.target(UI_LAYER);
            draw_batch.print_color(
                cmd.pt - camera.offset() + Point::new(0, -1 * offset),
                cmd.text.clone(),
                ColorPair::new(RGBA::from_f32(1.0, 0.0, 0.0, fg), BLACK));
            draw_batch.submit(4000).expect("error rendering fading text");
        }
    } else {
        commands.remove(*entity);
    }
}