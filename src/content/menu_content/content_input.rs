use graphics::*;
use winit::{
    event::*,
    keyboard::*,
};
use hecs::World;

use crate::{ContentType, DrawSetting, content::*, MouseInputType, socket::*};

mod register_input;
mod login_input;

use register_input::*;
use login_input::*;

impl MenuContent {
    pub fn mouse_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        socket: &mut Socket,
        input_type: MouseInputType,
        screen_pos: Vec2,
    ) {
        match &mut content.holder {
            ContentHolder::Menu(data) => {
                match data.cur_window {
                    WindowType::Register => {
                        register_mouse_input(data, world, systems, socket, input_type, screen_pos);
                    }
                    WindowType::Login => {
                        login_mouse_input(data, world, systems, socket, input_type, screen_pos);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn key_input(
        content: &mut Content,
        world: &mut World,
        systems: &mut DrawSetting,
        _socket: &mut Socket,
        event: &KeyEvent,
    ) {
        match &mut content.holder {
            ContentHolder::Menu(data) => {
                match data.cur_window {
                    WindowType::Register => {
                        register_key_input(data, world, systems, event);
                    }
                    WindowType::Login => {
                        login_key_input(data, world, systems, event);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}