use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    ComputerTurn,
}

pub struct State {
    ecs: World,
    resources: Resources,
    input_schedule: Schedule,
    computer_schedule: Schedule,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickCount(usize);

impl TickCount {
    #[inline]
    pub fn act(&self, n: usize) -> bool { self.0 % n == 0 }

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
        let mut state = Self{
            ecs: World::default(),
            resources: Resources::default(),
            input_schedule: build_input_schedule(),
            computer_schedule: build_computer_schedule(),
         };

        state.load_level(0);
        state
    }

    fn load_level(&mut self, depth: i32) {
        let mut rng = Rng::new();
        let mb = MapBuilder::build(&mut rng, depth);

        spawn_player(&mut self.ecs, mb.player_start);
        mb.monster_spawns.iter().for_each(|pt| spawn_monster(&mut self.ecs, &mut rng, *pt, depth));
        mb.rooms.iter().for_each(|rect| spawn_items(&mut self.ecs, &mut rng, *rect, depth));

        self.resources.insert(TickCount(0));
        self.resources.insert(mb.map);
        self.resources.insert(Camera::new(mb.player_start));
        self.resources.insert(TurnState::AwaitingInput);
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
        draw_batch.print_color_centered(
            2,
            "This game is under construction...",
            ColorPair::new(YELLOW, BLACK));


        draw_batch.draw_hollow_box(Rect::with_size(3, 20, 10, 5), ColorPair::new(RGBA::from_f32(1., 1., 1., 0.76), BLACK));
        draw_batch.draw_double_box(Rect::with_size(3, 30, 10, 5), ColorPair::new(WHITE, BLACK));
        draw_batch.draw_hollow_double_box(Rect::with_size(3, 40, 10, 5), ColorPair::new(RGBA::from_f32(1., 1., 1., 0.76), BLACK));

        // Draw status
        draw_batch.draw_box(Rect::with_size(3, 3, 20, 5), ColorPair::new(WHITE, BLACK));
        draw_batch.bar_horizontal(Point::new(10, 4), 5, 15, 23, ColorPair::new(RED, BLACK));
        draw_batch.bar_horizontal(Point::new(15, 4), 5, 8, 29, ColorPair::new(PINK, BLACK));
        draw_batch.print_right(Point::new(10, 4), "Health");
        draw_batch.bar_horizontal(Point::new(10, 5), 10, 15, 27, ColorPair::new(BLUE, BLACK));
        draw_batch.print_right(Point::new(10, 5), "Mana");
        draw_batch.submit(5000).expect("batch error in drawing UI layer");

        let curr_tick = self.resources.get::<TickCount>().unwrap().clone();
        curr_tick.msg(1000, "1000 ticks");

        self.resources.insert(TickCount(curr_tick.0 + 1));
        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let curr_state = self.resources.get::<TurnState>().unwrap().clone();
        match curr_state {
            TurnState::AwaitingInput => self.input_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::ComputerTurn => self.computer_schedule.execute(&mut self.ecs, &mut self.resources),
        };

        render_draw_buffer(ctx).expect("render error from draw buffer in tick");
    }
}