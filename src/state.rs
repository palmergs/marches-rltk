use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnState {
    InitializeMap,
    AwaitingInput,
    ComputerTurn,
    GameOver,
}

pub struct State {
    ecs: World,
    resources: Resources,
    initialize_schedule: Schedule,
    input_schedule: Schedule,
    computer_schedule: Schedule,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickCount(usize);

impl TickCount {
    #[inline]
    pub fn act(&self, n: usize) -> bool { self.0 % n == 0 }
    pub fn increment(&mut self) { self.0 += 1; }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurnCount(usize);

impl TurnCount {
    #[inline]
    pub fn act(&self, n: usize) -> bool { self.0 % n == 0 }
    pub fn increment(&mut self) { self.0 += 1; }
}

impl State {
    pub fn new() -> Self {
        let mut state = Self{
            ecs: World::default(),
            resources: Resources::default(),
            initialize_schedule: build_initialize_schedule(),
            input_schedule: build_input_schedule(),
            computer_schedule: build_computer_schedule(),
         };

        state.load_level(0);
        state
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        self.load_level(0);
    }

    fn load_level(&mut self, depth: i32) {
        let mut rng = Rng::new();
        let mb = MapBuilder::build(&mut rng, depth);

        spawn_player(&mut self.ecs, mb.player_start);
        mb.monster_spawns.iter().for_each(|pt| spawn_monster(&mut self.ecs, &mut rng, *pt, depth));
        mb.rooms.iter().for_each(|rect| spawn_room_items(&mut self.ecs, &mut rng, &mb.map, *rect, depth));
        for _ in 0..20 { spawn_dropped_item(&mut self.ecs, &mut rng, &mb.map, depth); }
        
        self.resources.insert(TickCount(0));
        self.resources.insert(TurnCount(0));
        self.resources.insert(mb.map);
        self.resources.insert(Camera::new(mb.player_start));
        self.resources.insert(TurnState::InitializeMap);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(UI_LAYER);
        ctx.cls();
        ctx.print_color_centered(3, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK, "Slain by a monster, you hero's journey has come to an end.");
        ctx.print_color_centered(8, WHITE, BLACK, "Don't worry, you can always try again with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            ctx.cls();
            self.reset_game_state();
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

        self.resources.get_mut::<TickCount>().unwrap().increment();
        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let curr_state = self.resources.get::<TurnState>().unwrap().clone();
        match curr_state {
            TurnState::InitializeMap => self.initialize_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::AwaitingInput => self.input_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::ComputerTurn => self.computer_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
        };

        render_draw_buffer(ctx).expect("render error from draw buffer in tick");
    }
}

