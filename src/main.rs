extern crate bracket_lib;

mod components;
mod maps;
mod systems;
mod spawners;
mod cameras;
mod state;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub use RandomNumberGenerator as Rng;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub use crate::maps::*;
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use crate::spawners::*;
    pub use crate::cameras::*;
    pub use crate::state::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const MAP_WIDTH: usize = 200;
    pub const MAP_HEIGHT: usize = 200;
    pub const MAP_TILES: usize = MAP_WIDTH * MAP_HEIGHT;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT;

    // Terminal layers
    pub const FLOOR_LAYER:usize = 0;
    pub const ITEM_LAYER:usize = 1;
    pub const ACTOR_LAYER:usize = 2;
    pub const UI_LAYER:usize = 3;
}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("The Western Marches")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(16, 16)
        .with_resource_path("resources/")
        .with_font("unicode_trunc_graph_16x16.png", 16, 16)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "unicode_trunc_graph_16x16.png")
        .with_sparse_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "unicode_trunc_graph_16x16.png")
        .with_sparse_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "unicode_trunc_graph_16x16.png")
        .with_sparse_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "unicode_trunc_graph_16x16.png")
        .build()?;

    main_loop(context, State::new())
}
