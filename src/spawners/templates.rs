use crate::prelude::*;

use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;

#[derive(Clone, Debug, Deserialize)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub tiles: Vec<(usize, usize)>,
    pub field_of_view: i32,
    pub field_of_light: Option<i32>,
    pub armor: i32,
    pub speed: i32,
    pub power: i32,
    pub vigor: i32,
    pub focus: i32,
    pub brawn: i32,
    pub grace: i32,
    pub smart: i32,
    pub charm: i32,
    pub outlook: Outlook,
    pub strategy: MoveStrategy,
    pub talking: Option<Talk>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Talk {
    pub chance: i32,
    pub phrases: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Actors {
    actors: Vec<Actor>
}

impl Actors {
    pub fn load() -> Self {
        let file = File::open("resources/actors.ron")
            .expect("failed opening actors template file");
        from_reader(file)
            .expect("unable to load actor templates")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum ItemCategory {
    Jewelry,
    Treasure,
    Furniture,
    Consumable,
    Weapon { armor: i32, power: i32, speed: i32 },
}

#[derive(Clone, Debug, Deserialize)]
pub enum ItemEffect {
    Light(i32),
    Power(i32),
    Armor(i32),
    Speed(i32),
    Focus(i32),
    Vigor(i32),
    Replace(String),
    Spawn(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemAction {

    // if the item supports multiple actions, this can be used
    // in the UI for the player to select a chosed action
    command: Option<String>,

    // number of uses of this action
    uses: Option<i32>,

    // all the actions that the effect initiates
    effects: Vec<ItemEffect>,

    // text to display when the action takes place
    text: Option<String>,

    // chance in 1000
    chance: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String, 
    pub category: ItemCategory,
    pub tiles: Vec<(usize, usize)>,
    pub slots: Option<Vec<EquipmentSlot>>,
    pub field_of_light: Option<i32>,

    // when the item is grabbed or, if immobile, when bumped
    pub touch: Option<Vec<ItemAction>>,

    // when the item is equipped or worn 
    // (effect is reversed when unequipped)
    pub equip: Option<Vec<ItemAction>>,

    // when dropped, fired or thrown (affect takes place at the point
    // where the item lands)
    pub fire: Option<Vec<ItemAction>>,

    // when the item is activated, used or consumed
    pub activate: Option<Vec<ItemAction>>,

    // when the item is destroyed
    pub destroy: Option<Vec<ItemAction>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Items {
    items: Vec<Item>
}

impl Items {
    pub fn load() -> Self {
        let file = File::open("resources/items.ron")
            .expect("failed opening items template file");
        from_reader(file)
            .expect("unable to load items templates")
    }
}
