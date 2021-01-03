use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Render)]
#[read_component(Physical)]
#[read_component(Mental)]
#[read_component(Stats)]
#[read_component(Actor)]
#[read_component(Item)]
#[read_component(FieldOfView)]
pub fn character(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] tick: &TickCount,
) {
    let mut query = <(&Render, &Physical, &Mental, &Stats)>::query().filter(component::<Player>());
    let (render, physical, mental, stats) = query.iter(ecs).next().unwrap();

    let border_color = ColorPair::new(RGB::from_f32(0.25, 0.25, 0.25), BLACK);
    let label_color = ColorPair::new(RGB::from_f32(0.5, 0.5, 0.5), BLACK);

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(UI_LAYER);
    draw_batch.draw_double_box(Rect::with_size(1, 1, 12, 12), border_color);
    draw_batch.print(Point::new(2, 2), render.name.clone());

    draw_batch.print_color(Point::new(2, 4), "AC".to_string(), label_color);
    draw_batch.print(Point::new(5, 4), format!("{}", stats.armor));
    draw_batch.print_color(Point::new(7, 4), "SP".to_string(), label_color);
    draw_batch.print(Point::new(10, 4), format!("{}", stats.speed));

    draw_batch.print_color(Point::new(2, 6), "BRAWN".to_string(), label_color);
    draw_batch.print(Point::new(7, 6), format!("{:>+3}", physical.brawn.curr));
    draw_batch.print_color(Point::new(2, 7), "GRACE".to_string(), label_color);
    draw_batch.print(Point::new(7, 7), format!("{:>+3}", physical.grace.curr));
    draw_batch.print_color(Point::new(2, 8), "CHARM".to_string(), label_color);
    draw_batch.print(Point::new(7, 8), format!("{:>+3}", mental.charm.curr));
    draw_batch.print_color(Point::new(2, 9), "SMART".to_string(), label_color);
    draw_batch.print(Point::new(7, 9), format!("{:>+3}", mental.smart.curr));     

    draw_batch.print_color(Point::new(2, 11), "V".to_string(), label_color);
    draw_batch.bar_horizontal(Point::new(3, 11), 9, stats.vigor.curr, stats.vigor.max, ColorPair::new(PINK, BLACK));
    draw_batch.print_color(Point::new(2, 12), "F".to_string(), label_color);
    draw_batch.bar_horizontal(Point::new(3, 12), 9, stats.focus.curr, stats.focus.max, ColorPair::new(CYAN, BLACK));

    draw_batch.submit(9999).expect("batch error in drawing character");
}
