use crate::prelude::*;

pub mod utils;
pub mod equipment;

mod map_initialize;
mod input;
mod select_item;
mod select_equipped;
mod select_target;
mod render;
mod state_change;
mod move_strategy;
mod movement;
mod might_talk;
mod combat;
mod strategy;
mod heal;
mod fov;
mod fol;
mod display_text;
mod hud;
mod tooltip;

// Initializing the map before the first render and player input
pub fn build_initialize_schedule() -> Schedule {
    Schedule::builder()
        .add_system(map_initialize::map_initialize_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(state_change::state_change_system())
        .build()
}

// Awaiting keyboard input and creating new player commands
pub fn build_input_schedule() -> Schedule {
    Schedule::builder()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(hud::character_system())
        .add_system(tooltip::tooltip_system())
        .flush()
        .add_system(display_text::display_text_system())
        .flush()
        .add_system(input::player_input_system())
        .build()
}

pub fn build_select_item_schedule() -> Schedule {
    Schedule::builder()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(hud::character_system())
        .add_system(hud::inventory_system())
        .flush()
        .add_system(select_item::select_item_system())
        .build()
}

pub fn build_select_equipped_schedule() -> Schedule {
    Schedule::builder()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(hud::character_system())
        .add_system(hud::equipment_system())
        .flush()
        .add_system(select_equipped::select_equipped_system())
        .build()
}

pub fn build_select_target_schedule() -> Schedule {
    Schedule::builder()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(hud::character_system())
        .flush()
        .add_system(select_target::select_target_system())
        .build()
}

// Resolving movement and creating new npc commands
pub fn build_computer_schedule() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(strategy::strategy_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(heal::heal_system())
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(move_strategy::move_strategy_system())
        .add_system(might_talk::might_talk_system())
        .flush()
        .add_system(render::render_system())
        .add_system(hud::character_system())
        .add_system(display_text::display_text_system())
        .add_system(state_change::state_change_system())
        .build()
}