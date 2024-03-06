use graphics::*;

use indexmap::IndexSet;

pub mod content_input;
pub mod interface;

pub use content_input::*;
pub use interface::*;

use crate::{
    values::*,
    logic::*,
    Direction,
    DrawSetting,
    content::*,
    database::*,
};
use hecs::World;

mod player;
mod npc;
pub mod map;
mod camera;
pub mod entity;

use player::*;
use npc::*;
pub use map::*;
use camera::*;
pub use entity::*;

const KEY_ATTACK: usize = 0;
const KEY_MOVEUP: usize = 1;
const KEY_MOVELEFT: usize = 2;
const KEY_MOVEDOWN: usize = 3;
const KEY_MOVERIGHT: usize = 4;
const MAX_KEY: usize = 5;

pub struct GameContent {
    players: IndexSet<Entity>,
    npcs: IndexSet<Entity>,
    pub map: MapContent,
    camera: Camera,
    interface: Interface,
    keyinput: [bool; MAX_KEY],
    // Test
    myentity: Option<Entity>,
}

impl GameContent {
    pub fn new(world: &mut World, systems: &mut DrawSetting) -> Self {
        let mut content = GameContent {
            players: IndexSet::default(),
            npcs: IndexSet::default(),
            map: MapContent::new(systems),
            camera: Camera::new(Vec2::new(0.0, 0.0)),
            interface: Interface::new(systems),
            keyinput: [false; MAX_KEY],

            myentity: None,
        };

        // TEMP //
        let player = add_player(world, systems, Vec2::new(0.0, 0.0));
        content.myentity = Some(player.clone());
        content.players.insert(player);
        let npcentity = add_npc(world, systems, Vec2::new(3.0, 2.0));
        content.npcs.insert(npcentity);
        // ---

        update_camera(world, &mut content, systems);

        content
    }

    pub fn unload(&mut self, world: &mut World, systems: &mut DrawSetting) {
        for entity in self.players.iter() {
            unload_player(world, systems, entity);
        }
        for entity in self.npcs.iter() {
            unload_npc(world, systems, entity);
        }
        self.myentity = None;
        self.interface.unload(systems);
        self.map.unload(systems);
    }

    pub fn setup_map(&mut self, systems: &mut DrawSetting, database: &mut Database) {
        self.map.map_attribute.clear();
        for i in 0..9 {
            load_map_data(systems, &database.map[i], self.map.index[i]);

            self.map.map_attribute.push(
                MapAttributes {
                    attribute: database.map[i].attribute.clone(),
                }
            )
        }
    }

    pub fn handle_key_input(&mut self, world: &mut World, systems: &mut DrawSetting, seconds: f32) {
        for i in 0..MAX_KEY {
            if self.keyinput[i] {
                match i {
                    KEY_ATTACK => self.player_attack(world, systems, seconds),
                    KEY_MOVEDOWN => self.move_player(world, systems, &Direction::Down),
                    KEY_MOVELEFT => self.move_player(world, systems, &Direction::Left),
                    KEY_MOVEUP => self.move_player(world, systems, &Direction::Up),
                    KEY_MOVERIGHT => self.move_player(world, systems, &Direction::Right),
                    _ => {}
                }
            }
        }
    }

    // TEMP //
    pub fn move_player(
        &mut self,
        world: &mut World,
        systems: &mut DrawSetting,
        dir: &Direction,
    ) {
        let myentity = self.myentity.expect("Could not find myentity");
        move_player(world, systems, &myentity, self, &dir);
    }
    pub fn player_attack(
        &mut self,
        world: &mut World,
        systems: &mut DrawSetting,
        seconds: f32,
    ) {
        let myentity = self.myentity.expect("Could not find myentity");
        init_player_attack(world, systems, &myentity, seconds);
    }
    // ---
}

pub fn update_player(world: &mut World, systems: &mut DrawSetting, content: &mut GameContent, seconds: f32) {
    let players = content.players.clone();
    for entity in players.iter() {
        process_player_movement(world, systems, entity);
        process_player_attack(world, systems, entity, seconds)
    }
}

pub fn update_npc(world: &mut World, systems: &mut DrawSetting, content: &mut GameContent, seconds: f32) {
    let npcs = content.npcs.clone();
    for entity in npcs.iter() {
        process_npc_movement(world, systems, entity);
        process_npc_attack(world, systems, entity, seconds)
    }
}

pub fn update_camera(world: &mut World, content: &mut GameContent, systems: &mut DrawSetting) {
    let player_pos = if let Some(entity) = content.myentity {
        let pos_offset = world.get_or_panic::<PositionOffset>(&entity);
        let pos = world.get_or_panic::<Position>(&entity);
        (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32) + pos_offset.offset
    } else {
        Vec2::new(0.0, 0.0)
    };
    let adjust_pos = get_screen_center(&systems.size) - player_pos;
    content.camera.pos = adjust_pos;

    content.map.move_pos(systems, content.camera.pos);
    
    for entity in content.players.iter() {
        update_player_position(world, systems, &content.camera, entity);
    }
    for entity in content.npcs.iter() {
        update_npc_position(world, systems, &content.camera, entity);
    }
}