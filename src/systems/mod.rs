use crate::prelude::*;

mod player_input;
mod map_render;
mod actor_render;
mod item_render;

pub fn build_input_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(item_render::item_render_system())
        .add_system(actor_render::actor_render_system())
        .build()
}