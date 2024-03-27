use arboard::Clipboard;
use cosmic_text::{Attrs, Metrics};
use graphics::*;

const KEY_CTRL: usize = 0;
const MAX_KEY: usize = 1;

use winit::{event::*, keyboard::*};

use crate::{logic::*, widget::*, SystemHolder};

pub struct Textbox {
    visible: bool,
    pub text: String,
    text_index: usize,
    bg: usize,
    limit: usize,
    pub size: Vec2,
    pub pos: Vec3,
    adjust_x: f32,
    is_selected: bool,

    special_key_hold: [bool; MAX_KEY],
    hide_content: bool,
    z_step: (f32, i32),
    pub tooltip: Option<String>,
}

impl Textbox {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut SystemHolder,
        pos: Vec3,
        z_step: (f32, i32),
        size: Vec2,
        text_color: Color,
        render_layer: usize,
        limit: usize,
        selected_bg_color: Color,
        hide_content: bool,
        visible: bool,
        tooltip: Option<String>,
    ) -> Self {
        let mut rect = Rect::new(&mut systems.renderer, 0);
        rect.set_color(selected_bg_color)
            .set_position(pos)
            .set_size(size);
        let bg = systems.gfx.add_rect(rect, 0);
        systems.gfx.set_visible(bg, false);

        let text_data = create_label(
            systems,
            Vec3::new(pos.x, pos.y, pos.z.sub_f32(z_step.0, z_step.1)),
            size,
            Bounds::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y),
            text_color,
        );
        let text_index = systems.gfx.add_text(text_data, render_layer);
        systems.gfx.set_visible(text_index, visible);

        Textbox {
            visible,
            text: String::new(),
            text_index,
            bg,
            limit,
            size,
            pos,
            z_step,
            adjust_x: 0.0,
            is_selected: false,
            special_key_hold: [false; MAX_KEY],
            hide_content,
            tooltip,
        }
    }

    pub fn set_select(&mut self, systems: &mut SystemHolder, is_select: bool) {
        if self.is_selected == is_select || !self.visible {
            return;
        }
        self.is_selected = is_select;
        systems.gfx.set_visible(self.bg, self.is_selected);
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(self.bg);
        systems.gfx.remove_gfx(self.text_index);
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        systems.gfx.set_visible(self.bg, visible);
        systems.gfx.set_visible(self.text_index, visible);
    }

    pub fn set_z_order(&mut self, systems: &mut SystemHolder, z_order: f32) {
        self.pos.z = z_order;
        systems.gfx.set_pos(self.bg, self.pos);
        systems.gfx.set_pos(
            self.text_index,
            Vec3::new(
                self.pos.x,
                self.pos.y,
                self.pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
    }

    pub fn set_pos(&mut self, systems: &mut SystemHolder, new_pos: Vec2) {
        self.pos.x = new_pos.x;
        self.pos.y = new_pos.y;
        systems.gfx.set_pos(self.bg, self.pos);
        systems.gfx.set_pos(
            self.text_index,
            Vec3::new(
                self.pos.x,
                self.pos.y,
                self.pos.z.sub_f32(self.z_step.0, self.z_step.1),
            ),
        );
        systems.gfx.set_bound(
            self.text_index,
            Bounds::new(
                self.pos.x,
                self.pos.y,
                self.pos.x + self.size.x,
                self.pos.y + self.size.y,
            ),
        );
    }

    pub fn set_text(&mut self, systems: &mut SystemHolder, text: String) {
        self.text.clear();
        if !text.is_empty() {
            self.text.push_str(&text);
        }
        let msg = if self.hide_content {
            self.text.chars().map(|_| '*').collect()
        } else {
            self.text.clone()
        };
        systems
            .gfx
            .set_text(&mut systems.renderer, self.text_index, &msg);
        self.adjust_text(systems);
    }

    pub fn enter_text(
        &mut self,
        systems: &mut SystemHolder,
        event: &KeyEvent,
        numeric_only: bool,
    ) {
        if !self.visible {
            return;
        }

        match event.physical_key {
            PhysicalKey::Code(KeyCode::ControlLeft)
            | PhysicalKey::Code(KeyCode::ControlRight) => {
                self.special_key_hold[KEY_CTRL] = event.state.is_pressed();
            }
            _ => {}
        }

        if !event.state.is_pressed() || !self.is_selected {
            return;
        }

        let mut did_edit = false;
        if self.special_key_hold[KEY_CTRL] {
            if !numeric_only {
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyC) => {
                        set_clipboard_text(self.text.clone());
                    }
                    PhysicalKey::Code(KeyCode::KeyV) => {
                        self.text.push_str(&get_clipboard_text());
                        did_edit = true;
                    }
                    _ => {}
                }
            }
        } else {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::Backspace) => {
                    self.text.pop();
                    did_edit = true;
                }
                PhysicalKey::Code(KeyCode::Delete) => {
                    self.text.clear();
                    did_edit = true;
                }
                _ => {
                    if self.text.len() >= self.limit {
                        return;
                    }
                    if is_text(event) {
                        if let Some(char) = event.logical_key.to_text() {
                            let can_proceed = if numeric_only {
                                is_numeric(char)
                            } else {
                                true
                            };
                            if can_proceed {
                                self.text.push_str(char);
                            }
                        }
                        did_edit = true;
                    }
                }
            };
        }

        if did_edit {
            let msg = if self.hide_content {
                self.text.chars().map(|_| '*').collect()
            } else {
                self.text.clone()
            };
            systems
                .gfx
                .set_text(&mut systems.renderer, self.text_index, &msg);
            self.adjust_text(systems);
        }
    }

    pub fn adjust_text(&mut self, systems: &mut SystemHolder) {
        let adjust_x =
            (systems.gfx.get_measure(self.text_index).x - self.size.x).max(0.0);
        if self.adjust_x == adjust_x {
            return;
        }
        self.adjust_x = adjust_x;
        systems.gfx.set_pos(
            self.text_index,
            Vec3::new(self.pos.x - self.adjust_x, self.pos.y, self.pos.z),
        );
    }
}

pub fn is_text(event: &KeyEvent) -> bool {
    matches!(
        event.physical_key,
        PhysicalKey::Code(
            KeyCode::KeyA
                | KeyCode::KeyB
                | KeyCode::KeyC
                | KeyCode::KeyD
                | KeyCode::KeyE
                | KeyCode::KeyF
                | KeyCode::KeyG
                | KeyCode::KeyH
                | KeyCode::KeyI
                | KeyCode::KeyJ
                | KeyCode::KeyK
                | KeyCode::KeyL
                | KeyCode::KeyM
                | KeyCode::KeyN
                | KeyCode::KeyO
                | KeyCode::KeyP
                | KeyCode::KeyQ
                | KeyCode::KeyR
                | KeyCode::KeyS
                | KeyCode::KeyT
                | KeyCode::KeyU
                | KeyCode::KeyV
                | KeyCode::KeyW
                | KeyCode::KeyX
                | KeyCode::KeyY
                | KeyCode::KeyZ
                | KeyCode::Digit1
                | KeyCode::Digit2
                | KeyCode::Digit3
                | KeyCode::Digit4
                | KeyCode::Digit5
                | KeyCode::Digit6
                | KeyCode::Digit7
                | KeyCode::Digit8
                | KeyCode::Digit9
                | KeyCode::Digit0
                | KeyCode::Comma
                | KeyCode::Period
                | KeyCode::BracketLeft
                | KeyCode::BracketRight
                | KeyCode::Backquote
                | KeyCode::Minus
                | KeyCode::Equal
                | KeyCode::Quote
                | KeyCode::Backslash
                | KeyCode::Semicolon
                | KeyCode::Slash
                | KeyCode::Space,
        )
    )
}

pub fn is_numeric(char: &str) -> bool {
    char.trim().parse::<i64>().is_ok()
}

pub fn get_clipboard_text() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.get_text() {
        Ok(data) => data,
        Err(_) => String::new(),
    }
}

pub fn set_clipboard_text(message: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(message).unwrap();
}
