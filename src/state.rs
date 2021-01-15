use crate::prelude::*;

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnState {
    InitializeMap,
    NewLevel(i32),
    AwaitingInput,
    SelectingItem(VirtualKeyCode),
    SelectingEquipped(VirtualKeyCode),
    SelectingTarget(VirtualKeyCode, Option<Entity>),
    ComputerTurn,
    GameOver,
}

pub struct State {
    ecs: World,
    resources: Resources,
    initialize_schedule: Schedule,
    input_schedule: Schedule,
    select_item_schedule: Schedule,
    select_equipped_schedule: Schedule,
    select_target_schedule: Schedule,
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
            select_item_schedule: build_select_item_schedule(),
            select_equipped_schedule: build_select_equipped_schedule(),
            select_target_schedule: build_select_target_schedule(),
            computer_schedule: build_computer_schedule(),
         };

        state.resources.insert(TickCount(0));
        state.resources.insert(TurnCount(0));

        spawn_player(&mut state.ecs, Point::zero());
        state.load_level(0);
        state
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        self.resources.insert(TickCount(0));
        self.resources.insert(TurnCount(0));

        spawn_player(&mut self.ecs, Point::zero());
        self.load_level(0);
    }

    fn load_level(&mut self, depth: i32) {
        println!("in load level.... {:?}", depth);
        let mut rng = Rng::new();
        let player_entity = *<Entity>::query().filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();
            
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        <Entity>::query()
            .filter(component::<Carried>())
            .iter(&self.ecs)
            .for_each(|entity| { entities_to_keep.insert(*entity); });

        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        &cb.flush(&mut self.ecs);

        let mb = MapBuilder::build(&mut rng, depth);
        mb.monster_spawns.iter().for_each(|pt| {
            spawn_monster(&mut self.ecs, &mut rng, *pt, depth); 
        });
        mb.rooms.iter().for_each(|rect| {
            spawn_room_items(&mut self.ecs, &mut rng, &mb.map, *rect, depth); 
        });
        spawn_map_items(&mut self.ecs, &mut rng, &mb.map, 100, depth);
        spawn("torch", &mut self.ecs, mb.player_start + Point::new(1,0));
        spawn("seltzer", &mut self.ecs, mb.player_start + Point::new(-1,0));
        spawn("healing potion", &mut self.ecs, mb.player_start + Point::new(0,1));
        if mb.rooms.len() > 1 {
            for _ in 0..10 {
                if depth < 0 {
                    let rect = &mb.rooms[rng.range(0, mb.rooms.len())];
                    spawn_room_stairs_up(&mut self.ecs, &mut rng, &mb.map, rect, depth + 1);
                }
                let rect = &mb.rooms[rng.range(0, mb.rooms.len())];
                spawn_room_stairs_down(&mut self.ecs, &mut rng, &mb.map, rect, depth - 1);
            }
        } else {
            for _ in 0..10 {
                if depth < 0  {
                    spawn_map_stairs_up(&mut self.ecs, &mut rng, &mb.map, depth + 1);
                }
                spawn_map_stairs_down(&mut self.ecs, &mut rng, &mb.map, depth + 1);
            }
        }

        <(&mut Player, &mut Point, &mut FieldOfView, &mut FieldOfLight)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pt, fov, fol)| {
                player.depth = depth;
                *pt = mb.player_start;
                fov.is_dirty = true;
                fol.is_dirty = true;
            });
        
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
            TurnState::NewLevel(depth) => self.load_level(depth),
            TurnState::AwaitingInput => self.input_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::SelectingTarget(_, _) => self.select_target_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::SelectingItem(_) => self.select_item_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::SelectingEquipped(_) => self.select_equipped_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::ComputerTurn => self.computer_schedule.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
        };

        render_draw_buffer(ctx).expect("render error from draw buffer in tick");
    }
}

