use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Equipped)]
#[read_component(Carried)]
#[read_component(Render)]
#[read_component(Point)]
#[read_component(Item)]
pub fn select_item(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
) {
    println!("in select item...");
    if let Some(key2) = key {

        println!("key2 = {:?}", key2);
        let mut query = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player, location) = query.iter(ecs)
            .find_map(|(entity, pt)| Some((*entity, *pt))).unwrap();

        match key2 {
            VirtualKeyCode::Escape | VirtualKeyCode::Back | VirtualKeyCode::Delete => {
                println!("escaping back to awaiting input...");
                *state = TurnState::AwaitingInput;
                return
            },
            _ => ()
        }

        // let new_state = match state {
        //     TurnState::SelectingItem(key1) => {
        //         TurnState::ComputerTurn
        //     },
        //     _ => return
        // };

        // *state = new_state;
    }
}
