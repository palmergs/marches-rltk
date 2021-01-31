use crate::prelude::*;

#[system(for_each)]
pub fn display_text(
    entity: &Entity,
    cmd: &mut Text,
    commands: &mut CommandBuffer,
    #[resource] camera: &Camera,
) {
    if cmd.count < cmd.ticks {
        let fade = cmd.count as f32 / cmd.ticks as f32;
        let rgba = cmd.color - RGBA::from_f32(0., 0., 0., fade);

        cmd.count += 1;
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(UI_LAYER);
        match cmd.display {
            TextDisplay::AnimateUp(pt) => {
                if camera.in_central_view(pt) {
                    let dy = (fade * 4.0) as i32;
                    let pt = pt - camera.offset() - Point::new(0, dy);
                    draw_batch.print_color(
                        pt * 4 + Point::new(1,0), 
                        cmd.text.clone(), 
                        ColorPair::new(rgba, BLACK));
                }
            },
            TextDisplay::Fade(pt) => {
                if camera.in_central_view(pt) {
                    let pt = pt - camera.offset();
                    draw_batch.print_color(
                        pt * 4, 
                        cmd.text.clone(), 
                        ColorPair::new(rgba, BLACK));
                }
            },
        }
        draw_batch.submit(4000).expect("error rendering fading text");

    } else {
        commands.remove(*entity);
    }
}
