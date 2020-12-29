use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnState {
    AwaitingInput
}

pub struct State {
    ecs: World,
    resources: Resources,
    input_schedule: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();

        let player_start = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        spawn_player(&mut ecs, player_start);

        let mut resources = Resources::default();
        resources.insert(Map::new());
        resources.insert(Camera::new(player_start));
        resources.insert(TurnState::AwaitingInput);

        Self{
            ecs,
            resources,
            input_schedule: build_input_schedule(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(UI_LAYER);
        ctx.cls();

        ctx.set_active_console(ACTOR_LAYER);
        ctx.cls();

        ctx.set_active_console(ITEM_LAYER);
        ctx.cls();

        ctx.set_active_console(FLOOR_LAYER);
        ctx.cls();

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(UI_LAYER);
        draw_batch.print_color_centered(2, "This game is under construction...", ColorPair::new(BLUE, BLACK));
        draw_batch.submit(0).expect("drawing UI layer");

        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let curr_state = self.resources.get::<TurnState>().unwrap().clone();
        match curr_state {
            TurnState::AwaitingInput => self.input_schedule.execute(&mut self.ecs, &mut self.resources),
        };

        render_draw_buffer(ctx).expect("render error from draw buffer in tick");
    }
}