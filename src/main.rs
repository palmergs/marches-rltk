extern crate bracket_lib;

mod components;
mod maps;
mod systems;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub use RandomNumberGenerator as Rng;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub use crate::maps::*;
    pub use crate::components::*;
    pub use crate::systems::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const MAP_WIDTH: i32 = 200;
    pub const MAP_HEIGHT: i32 = 200;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT;

    // Terminal layers
    pub const FLOOR:usize = 0;
    pub const ITEMS:usize = 1;
    pub const CHARS:usize = 2;
    pub const UI:usize = 3;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();

        Self{
            ecs,
            resources,
        }
    }
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(UI);
        ctx.cls();

        ctx.set_active_console(CHARS);
        ctx.cls();

        ctx.set_active_console(ITEMS);
        ctx.cls();

        ctx.set_active_console(FLOOR);
        ctx.cls();

        ctx.set_active_console(UI);
        ctx.print_color_centered(2, GREEN, BLACK, "This game is under construction...");

        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        render_draw_buffer(ctx).expect("render error from draw buffer in tick");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("The Western Marches")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(16, 16)
        .with_resource_path("resources/")
        .with_font("roguelikecreatures.png", 16, 16)
        .with_font("roguelikeitems.png", 16, 16)
        .with_font("rogueliketiles.png", 16, 16)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "rogueliketiles.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "roguelikeitems.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "roguelikecreatures.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
