use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Physical)]
#[read_component(Mental)]
#[read_component(Stats)]
#[read_component(Actor)]
#[read_component(FieldOfView)]
pub fn character(
    ecs: &SubWorld,
) {
    let mut query = <(&Render, &Physical, &Mental, &Stats, &Player)>::query();
    let (render, physical, mental, stats, player) = query.iter(ecs).next().unwrap();

    let border_color = ColorPair::new(RGB::from_f32(0.25, 0.25, 0.25), BLACK);
    let label_color = ColorPair::new(RGB::from_f32(0.5, 0.5, 0.5), BLACK);

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(UI_LAYER);

    draw_batch.draw_double_box(Rect::with_size(1, 1, 20, 15), border_color);
    draw_batch.print(Point::new(2, 2), render.name.clone());

    draw_batch.print_color(Point::new(2, 4), "Exploring".to_string(), label_color);
    draw_batch.print(Point::new(12, 4), format!("{}", player.depth.abs()));

    draw_batch.print_color(Point::new(2, 6), "AC".to_string(), label_color);
    draw_batch.print(Point::new(5, 6), format!("{}", stats.armor));
    draw_batch.print_color(Point::new(7, 6), "SP".to_string(), label_color);
    draw_batch.print(Point::new(10, 6), format!("{}", stats.speed));
    draw_batch.print_color(Point::new(12, 6), "PW".to_string(), label_color);
    draw_batch.print(Point::new(15, 6), format!("{}", stats.power));

    draw_batch.print_color(Point::new(2, 8), "BRAWN".to_string(), label_color);
    draw_batch.print(Point::new(7, 8), format!("{:>+3}", physical.brawn.curr));
    draw_batch.print_color(Point::new(2, 9), "GRACE".to_string(), label_color);
    draw_batch.print(Point::new(7, 9), format!("{:>+3}", physical.grace.curr));
    draw_batch.print_color(Point::new(2, 10), "CHARM".to_string(), label_color);
    draw_batch.print(Point::new(7, 10), format!("{:>+3}", mental.charm.curr));
    draw_batch.print_color(Point::new(2, 11), "SMART".to_string(), label_color);
    draw_batch.print(Point::new(7, 11), format!("{:>+3}", mental.smart.curr));     

    draw_batch.print_color(Point::new(2, 13), "VIGOR".to_string(), label_color);
    draw_batch.bar_horizontal(Point::new(8, 13), 12, stats.vigor.curr, stats.vigor.max, ColorPair::new(PINK, BLACK));
    draw_batch.print_color(Point::new(2, 14), "FOCUS".to_string(), label_color);
    draw_batch.bar_horizontal(Point::new(8, 14), 12, stats.focus.curr, stats.focus.max, ColorPair::new(CYAN, BLACK));

    let mut query = <&FieldOfView>::query().filter(component::<Player>());
    let fov = query.iter(ecs).next().unwrap();

    let mut actor_count = 0;
    draw_batch.draw_double_box(Rect::with_size(1, 17, 20, 20), border_color);
    draw_batch.print_color(Point::new(2, 18), "Visible".to_string(), label_color);
    let mut query = <(&Point, &Render, &Stats)>::query().filter(!component::<Player>());
    query.iter(ecs)
        .filter(|(pt, _, _)| fov.visible_tiles.contains(pt) )
        .for_each(|(_, render, stats)| {
            if actor_count < 18 {
                actor_count += 1;
                if stats.vigor.is_wounded() {
                    draw_batch.print_color(
                        Point::new(2, 19 + actor_count), 
                        render.name.clone(), 
                        ColorPair::new(PINK, BLACK));
                } else {
                    draw_batch.print_color(
                        Point::new(2, 19 + actor_count), 
                        render.name.clone(), 
                        ColorPair::new(WHITE, BLACK));
                }
            }
        });

    draw_batch.submit(9999).expect("batch error in drawing character");
}
