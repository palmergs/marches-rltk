use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    ComputerTurn,
}

pub struct State {
    ecs: World,
    resources: Resources,
    input_schedule: Schedule,
    player_schedule: Schedule,
    computer_schedule: Schedule,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickCount(usize);

impl TickCount {
    pub fn msg(&self, n: usize, s: &str) -> bool { 
        if self.0 % n == 0 { 
            println!("{}", s); 
            return true;
        }

        false
    }
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();

        let player_start = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        spawn_player(&mut ecs, player_start);

        let mut rng = Rng::new();
        for _ in 0..200 {
            spawn_torch(&mut ecs, Point::new(rng.range(1, MAP_WIDTH - 1), rng.range(1, MAP_HEIGHT - 1)));
        }

        for _ in 0..200 {
            match rng.range(0, 4) {
                0 => spawn_goblin_with_torch(
                    &mut ecs,
                    Point::new(rng.range(1, MAP_WIDTH - 1), rng.range(1, MAP_HEIGHT - 1))),
                1 => spawn_animated_tree(
                    &mut ecs,
                    Point::new(rng.range(1, MAP_WIDTH - 1), rng.range(1, MAP_HEIGHT - 1))),                    
                _ => spawn_rat(
                    &mut ecs,
                    Point::new(rng.range(1, MAP_WIDTH - 1), rng.range(1, MAP_HEIGHT - 1))),
            }
        }

        let mut resources = Resources::default();
        resources.insert(TickCount(0));
        resources.insert(Map::new());
        resources.insert(Camera::new(player_start));
        resources.insert(TurnState::AwaitingInput);

        Self{
            ecs,
            resources,
            input_schedule: build_input_schedule(),
            player_schedule: build_player_schedule(),
            computer_schedule: build_computer_schedule(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(UI_LAYER);
        draw_batch.cls();
        draw_batch.print_color_centered(
            2,
            "This game is under construction...", 
            ColorPair::new(YELLOW, BLACK));
        draw_batch.submit(5000).expect("batch error in drawing UI layer");

        let curr_tick = self.resources.get::<TickCount>().unwrap().clone();
        curr_tick.msg(1000, "1000 ticks");

        self.resources.insert(TickCount(curr_tick.0 + 1));
        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let curr_state = self.resources.get::<TurnState>().unwrap().clone();
        match curr_state {
            TurnState::AwaitingInput => self.input_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::ComputerTurn => self.computer_schedule.execute(&mut self.ecs, &mut self.resources),
        };

        render_draw_buffer(ctx).expect("render error from draw buffer in tick");
    }
}