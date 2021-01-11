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
#[read_component(Equipped)]
#[read_component(Carried)]
pub fn character(
    ecs: &SubWorld,
) {    
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(UI_LAYER);
    draw_character(ecs, &mut draw_batch, &Rect::with_size(1, 1, 20, 15));
    draw_equipment(ecs, &mut draw_batch, &Rect::with_size(1, 17, 20, 20));
    draw_visible(ecs, &mut draw_batch, &Rect::with_size(1, 38, 20, 20));
    // draw_inventory(ecs, &mut draw_batch, &Rect::with_size(1, 38, 20, 20));
    draw_batch.submit(9999).expect("batch error in drawing character");
}

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Equipped)]
pub fn inventory(
    ecs: &SubWorld,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(UI_LAYER);
    draw_inventory_select(ecs, &mut draw_batch, &Rect::with_size(DISPLAY_WIDTH - 32, 1, 30, 30));
    draw_batch.submit(9999).expect("batch error in drawing inventory select");
}

fn draw_character(ecs: &SubWorld, draw_batch: &mut DrawBatch, rect: &Rect) {
    let border_color: ColorPair = ColorPair::new(RGB::from_f32(0.25, 0.25, 0.25), BLACK);
    let label_color: ColorPair = ColorPair::new(RGB::from_f32(0.5, 0.5, 0.5), BLACK);

    let mut query = <(&Render, &Physical, &Mental, &Stats, &Player)>::query();
    let (render, physical, mental, stats, player) = query.iter(ecs).next().unwrap();

    draw_batch.draw_double_box(*rect, border_color);
    let x = rect.x1 + 1;
    let y = rect.y1 + 1;
    draw_batch.print(Point::new(x, y), render.name.clone());

    draw_batch.print_color(Point::new(x, y + 2), "Exploring".to_string(), label_color);
    draw_batch.print(Point::new(12, 4), format!("{}", player.depth.abs()));

    draw_batch.print_color(Point::new(x, y + 4), "AC".to_string(), label_color);
    draw_batch.print(Point::new(x + 3, y + 4), format!("{}", stats.armor));
    draw_batch.print_color(Point::new(x + 5, y + 4), "SP".to_string(), label_color);
    draw_batch.print(Point::new(x + 8, y + 4), format!("{}", stats.speed));
    draw_batch.print_color(Point::new(x + 10, y + 4), "PW".to_string(), label_color);
    draw_batch.print(Point::new(x + 13, y + 4), format!("{}", stats.power));

    draw_batch.print_color(Point::new(x, y + 6), "BRAWN".to_string(), label_color);
    draw_batch.print(Point::new(x + 7, y + 6), format!("{:>+3}", physical.brawn.curr));
    draw_batch.print_color(Point::new(x, y + 7), "GRACE".to_string(), label_color);
    draw_batch.print(Point::new(x + 7, y + 7), format!("{:>+3}", physical.grace.curr));
    draw_batch.print_color(Point::new(x, y + 8), "CHARM".to_string(), label_color);
    draw_batch.print(Point::new(x + 7, y + 8), format!("{:>+3}", mental.charm.curr));
    draw_batch.print_color(Point::new(x, y + 9), "SMART".to_string(), label_color);
    draw_batch.print(Point::new(x + 7, y + 9), format!("{:>+3}", mental.smart.curr));     

    draw_batch.print_color(Point::new(x, y + 11), "VIGOR".to_string(), label_color);
    draw_batch.bar_horizontal(
        Point::new(x + 6, y + 11), 
        12, 
        stats.vigor.curr, 
        stats.vigor.max, 
        ColorPair::new(PINK, BLACK));

    draw_batch.print_color(Point::new(x, y + 12), "FOCUS".to_string(), label_color);
    draw_batch.bar_horizontal(
        Point::new(x + 6, y + 12), 
        12, 
        stats.focus.curr, 
        stats.focus.max, 
        ColorPair::new(CYAN, BLACK));
}

fn draw_visible(ecs: &SubWorld, draw_batch: &mut DrawBatch, rect: &Rect) {
    let border_color: ColorPair = ColorPair::new(RGB::from_f32(0.25, 0.25, 0.25), BLACK);
    let label_color: ColorPair = ColorPair::new(RGB::from_f32(0.5, 0.5, 0.5), BLACK);

    let mut query = <&FieldOfView>::query().filter(component::<Player>());
    let fov = query.iter(ecs).next().unwrap();

    let mut actor_count = 0;
    draw_batch.draw_double_box(*rect, border_color);
    let x = rect.x1 + 1;
    let y = rect.y1 + 1;
    draw_batch.print_color(Point::new(x, y), "Visible".to_string(), label_color);
    let mut query = <(&Point, &Render, &Stats)>::query().filter(!component::<Player>());
    query.iter(ecs)
        .filter(|(pt, _, _)| fov.visible_tiles.contains(pt) )
        .for_each(|(_, render, stats)| {
            if actor_count < 18 {
                actor_count += 1;
                if stats.vigor.is_wounded() {
                    draw_batch.print_color(
                        Point::new(x, y + 2 + actor_count), 
                        render.name.clone(), 
                        ColorPair::new(PINK, BLACK));
                } else {
                    draw_batch.print_color(
                        Point::new(x, y + 2 + actor_count), 
                        render.name.clone(), 
                        ColorPair::new(WHITE, BLACK));
                }
            }
        });
}

fn draw_inventory_select(ecs: &SubWorld, draw_batch: &mut DrawBatch, rect: &Rect) {
    let border_color: ColorPair = ColorPair::new(RGB::from_f32(0.75, 0.75, 0.75), BLACK);
    let label_color: ColorPair = ColorPair::new(RGB::from_f32(0.75, 0.75, 0.75), BLACK);

    let inventory = list_of_items(ecs);

    draw_batch.draw_double_box(*rect, border_color);
    let x = rect.x1 + 1;
    let y = rect.y1 + 1;
    draw_batch.print_color(Point::new(x, y), "Select Item (a-z)".to_string(), label_color);
    let mut item_offset = 2;
    let mut item_char = b'a';
    for (name, _, count) in inventory.iter() {
        if item_offset < 18 {
            if *count > 1 {
                draw_batch.print(
                    Point::new(x, y + item_offset), 
                    format!("{}. {} ({})", 
                    item_char as char, name, count));
            } else {
                draw_batch.print(
                    Point::new(x, y + item_offset), 
                    format!("{}. {}", 
                    item_char as char, name));
            }
        }
        item_char += 1;
        item_offset += 1;
    }
}

fn draw_equipment(ecs: &SubWorld, draw_batch: &mut DrawBatch, rect: &Rect) {
    let border_color: ColorPair = ColorPair::new(RGB::from_f32(0.25, 0.25, 0.25), BLACK);
    let label_color: ColorPair = ColorPair::new(RGB::from_f32(0.5, 0.5, 0.5), BLACK);

    draw_batch.draw_double_box(*rect, border_color);
    let x = rect.x1 + 1;
    let mut y = rect.y1 + 1;
    draw_batch.print_color(Point::new(x, y), "Equipment".to_string(), label_color);

    <(&Equipped, &Render)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .for_each(|(equipped, render)| {
            y += 1;
            draw_batch.print(Point::new(x, y), format!("{} ({:?})", render.name, equipped.slot));
        });
}
