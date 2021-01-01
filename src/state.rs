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

        ecs.push(
            (
                FadingText{
                    text: "Welcome to the Dungeon!".to_string(), 
                    life: 100, 
                    remaining: 100, 
                    pt: player_start },
            )
        );

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
            TurnState::PlayerTurn => self.player_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::ComputerTurn => self.computer_schedule.execute(&mut self.ecs, &mut self.resources),
        };

        render_draw_buffer(ctx).expect("render error from draw buffer in tick");
    }
}