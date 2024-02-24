use graphics::*;

pub mod content_input;

pub use content_input::*;

use crate::{
    content::*,
    gfx_order::*,
    DrawSetting,
    SCREEN_HEIGHT, SCREEN_WIDTH,
    interface::*,
};
use hecs::World;

pub enum WindowType {
    None,
    Login,
    Register,
}

pub struct MenuContent {
    bg: usize,
    cur_window: WindowType,
    window: Vec<usize>,
    label: Vec<usize>,
    unique_label: Vec<usize>,
    button: Vec<Button>,
    checkbox: Vec<Checkbox>,
    textbox: Vec<Textbox>,
    selected_textbox: Option<usize>,

    pub did_button_click: bool,
    pub did_checkbox_click: bool,
}

impl MenuContent {
    pub fn new(_world: &mut World, systems: &mut DrawSetting) -> Self {
        let mut bg_image = Image::new(Some(systems.resource.menu_bg.allocation),
            &mut systems.renderer, 0);
        bg_image.pos = Vec3::new(0.0, 0.0, MENU_BG);
        bg_image.hw = Vec2::new(800.0, 600.0);
        bg_image.uv = Vec4::new(0.0, 0.0, 800.0, 600.0);
        let bg = systems.gfx.add_image(bg_image, 0);

        let mut content = MenuContent {
            bg,
            cur_window: WindowType::None,
            window: Vec::new(),
            label: Vec::new(),
            unique_label: Vec::new(),
            button: Vec::new(),
            checkbox: Vec::new(),
            textbox: Vec::new(),
            did_button_click: false,
            did_checkbox_click: false,
            selected_textbox: None,
        };

        create_window(systems, &mut content, WindowType::Login);

        content
    }

    pub fn unload(&mut self, _world: &mut World, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.bg);
        self.window.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(*gfx_index);
        });
        self.label.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(*gfx_index);
        });
        self.unique_label.iter().for_each(|gfx_index| {
            systems.gfx.remove_gfx(*gfx_index);
        });
        self.button.iter_mut().for_each(|button| {
            button.unload(systems);
        });
        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.unload(systems);
        });
        self.textbox.iter_mut().for_each(|textbox| {
            textbox.unload(systems);
        });
        self.window.clear();
        self.button.clear();
        self.label.clear();
        self.unique_label.clear();
        self.checkbox.clear();
        self.textbox.clear();
        self.selected_textbox = None;
    }
}

pub fn hover_buttons(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    screen_pos: Vec2,
) {
    for button in menu_content.button.iter_mut() {
        if screen_pos.x >= button.pos.x &&
            screen_pos.x <= button.pos.x + button.size.x &&
            screen_pos.y >= button.pos.y &&
            screen_pos.y <= button.pos.y + button.size.y {
            button.set_hover(systems, true);
        } else {
            button.set_hover(systems, false);
        }
    }
}

pub fn click_buttons(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    screen_pos: Vec2,
) -> Option<usize> {
    let mut button_found = None;
    for (index, button) in menu_content.button.iter_mut().enumerate() {
        if screen_pos.x >= button.pos.x &&
            screen_pos.x <= button.pos.x + button.size.x &&
            screen_pos.y >= button.pos.y &&
            screen_pos.y <= button.pos.y + button.size.y {
            button.set_click(systems, true);
            button_found = Some(index)
        }
    }
    button_found
}

pub fn reset_buttons(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
) {
    if !menu_content.did_button_click {
        return;
    }
    menu_content.did_button_click = false;

    menu_content.button.iter_mut().for_each(|button| {
        button.set_click(systems, false);
    });
}

pub fn hover_checkbox(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    screen_pos: Vec2,
) {
    for checkbox in menu_content.checkbox.iter_mut() {
        if screen_pos.x >= checkbox.pos.x &&
            screen_pos.x <= checkbox.pos.x + checkbox.box_size.x + checkbox.adjust_x &&
            screen_pos.y >= checkbox.pos.y &&
            screen_pos.y <= checkbox.pos.y + checkbox.box_size.y {
            checkbox.set_hover(systems, true);
        } else {
            checkbox.set_hover(systems, false);
        }
    }
}

pub fn click_checkbox(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    screen_pos: Vec2,
) -> Option<usize> {
    let mut checkbox_found = None;
    for (index, checkbox) in menu_content.checkbox.iter_mut().enumerate() {
        if screen_pos.x >= checkbox.pos.x &&
            screen_pos.x <= checkbox.pos.x + checkbox.box_size.x + checkbox.adjust_x &&
            screen_pos.y >= checkbox.pos.y &&
            screen_pos.y <= checkbox.pos.y + checkbox.box_size.y {
            checkbox.set_click(systems, true);
            checkbox_found = Some(index)
        }
    }
    checkbox_found
}

pub fn reset_checkbox(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
) {
    if !menu_content.did_checkbox_click {
        return;
    }
    menu_content.did_checkbox_click = false;

    menu_content.checkbox.iter_mut().for_each(|checkbox| {
        checkbox.set_click(systems, false);
    });
}

pub fn click_textbox(
    menu_content: &mut MenuContent,
    systems: &mut DrawSetting,
    screen_pos: Vec2,
) {
    let mut checkbox_found = None;
    for (index, textbox) in menu_content.textbox.iter_mut().enumerate() {
        if screen_pos.x >= textbox.pos.x &&
            screen_pos.x <= textbox.pos.x + textbox.size.x &&
            screen_pos.y >= textbox.pos.y &&
            screen_pos.y <= textbox.pos.y + textbox.size.y {
            textbox.set_select(systems, true);
            checkbox_found = Some(index)
        }
    }
    if let Some(index)  = menu_content.selected_textbox {
        menu_content.textbox[index].set_select(systems, false);
    }
    if let Some(index) = checkbox_found {
        menu_content.textbox[index].set_select(systems, true);
    }
    menu_content.selected_textbox = checkbox_found;
}

pub fn create_window(systems: &mut DrawSetting, content: &mut MenuContent, window_type: WindowType) {
    content.cur_window = window_type;
    content.window.iter().for_each(|gfx_index| {
        systems.gfx.remove_gfx(*gfx_index);
    });
    content.button.iter_mut().for_each(|button| {
        button.unload(systems);
    });
    content.label.iter().for_each(|gfx_index| {
        systems.gfx.remove_gfx(*gfx_index);
    });
    content.unique_label.iter().for_each(|gfx_index| {
        systems.gfx.remove_gfx(*gfx_index);
    });
    content.checkbox.iter_mut().for_each(|checkbox| {
        checkbox.unload(systems);
    });
    content.textbox.iter_mut().for_each(|textbox| {
        textbox.unload(systems);
    });
    content.window.clear();
    content.button.clear();
    content.label.clear();
    content.unique_label.clear();
    content.checkbox.clear();
    content.textbox.clear();
    content.selected_textbox = None;

    let screen_size = Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    match content.cur_window {
        WindowType::Login => {
            let size = Vec2::new(348.0, 226.0);
            let pos = Vec2::new((screen_size.x - size.x) * 0.5, 80.0);

            let mut menu_rect = Rect::new(&mut systems.renderer, 0);
            menu_rect.set_position(Vec3::new(pos.x - 1.0, pos.y - 1.0, MENU_WINDOW))
                .set_size(size + 2.0)
                .set_color(Color::rgba(160, 160, 160, 255))
                .set_border_color(Color::rgba(10, 10, 10, 255))
                .set_border_width(1.0);
            content.window.push(systems.gfx.add_rect(menu_rect, 0));

            let mut header_rect = Rect::new(&mut systems.renderer, 0);
            header_rect.set_position(Vec3::new(pos.x, pos.y + 196.0, MENU_WINDOW_CONTENT))
                .set_size(Vec2::new(size.x, 30.0))
                .set_color(Color::rgba(120, 120, 120, 255));
            content.window.push(systems.gfx.add_rect(header_rect, 0));

            let header_text = create_label(systems, 
                Vec3::new(pos.x, pos.y + 199.0, MENU_WINDOW_CONTENT_DETAIL), 
                Vec2::new(size.x, 20.0),
                Bounds::new(pos.x, pos.y + 199.0, pos.x + size.x, pos.y + 219.0),
                Color::rgba(200, 200, 200, 255));
            let text_index = systems.gfx.add_text(header_text, 1);
            systems.gfx.set_text(&mut systems.renderer, text_index, "Login Window");
            systems.gfx.center_text(text_index);
            content.label.push(text_index);

            for index in 0..2 {
                let mut labelbox = Rect::new(&mut systems.renderer, 0);
                let mut textbox = Rect::new(&mut systems.renderer, 0);
                let addy = match index {
                    1 => 123.0,
                    _ => 154.0,
                };
                labelbox.set_position(Vec3::new(pos.x + 24.0, pos.y + addy, MENU_WINDOW_CONTENT))
                    .set_size(Vec2::new(116.0, 24.0))
                    .set_color(Color::rgba(208, 208, 208, 255));
                textbox.set_position(Vec3::new(pos.x + 140.0, pos.y + addy, MENU_WINDOW_CONTENT))
                    .set_size(Vec2::new(184.0, 24.0))
                    .set_color(Color::rgba(90, 90, 90, 255));
                content.window.push(systems.gfx.add_rect(labelbox, 0));
                content.window.push(systems.gfx.add_rect(textbox, 0));

                let tpos = Vec2::new(pos.x + 27.0, pos.y + addy + 2.0);
                let text = create_label(systems,
                    Vec3::new(tpos.x, tpos.y, MENU_WINDOW_CONTENT_DETAIL),
                    Vec2::new(110.0, 20.0),
                    Bounds::new(tpos.x, tpos.y, tpos.x + 110.0, tpos.y + 20.0),
                    Color::rgba(100, 100, 100, 255));
                let textindex = systems.gfx.add_text(text, 1);
                let msg = match index {
                    1 => "Password",
                    _ => "Email",
                };
                systems.gfx.set_text(&mut systems.renderer, textindex, msg);
                content.label.push(textindex);

                let is_hidden = match index {
                    1 => true,
                    _ => false,
                };

                let textbox = Textbox::new(systems,
                    Vec3::new(pos.x + 142.0, pos.y + addy + 2.0, MENU_WINDOW_CONTENT_DETAIL),
                    Vec2::new(180.0, 20.0),
                    Color::rgba(200, 200, 200, 255),
                    1,
                    255,
                    Color::rgba(120, 120, 120, 255),
                    is_hidden);
                content.textbox.push(textbox);
            }

            let button = Button::new(systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(70, 70, 70, 255),
                        border_radius: 0.0,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(180, 180, 180, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(40, 40, 40, 255)),
                    }),
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Login".to_string(),
                        pos: Vec3::new(0.0, 7.0, MENU_WINDOW_CONTENT),
                        color: Color::rgba(200, 200, 200, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(170, 170, 170, 255))
                    }),
                Vec3::new(pos.x + 104.0, pos.y + 45.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 34.0),
                0);
            content.button.push(button);

            let button = Button::new(systems,
                ButtonType::None,
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Register".to_string(),
                        pos: Vec3::new(0.0, 0.0, MENU_WINDOW_CONTENT),
                        color: Color::rgba(80, 80, 80, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(220, 220, 220, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                    }),
                Vec3::new(pos.x + 104.0, pos.y + 19.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 20.0),
                0);
            content.button.push(button);

            let checkbox = Checkbox::new(
                systems,
                CheckboxType::Rect(
                    CheckboxRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(50, 50, 50, 255),
                        border_radius: 2.0,
                        hover_change: CheckboxChangeType::ColorChange(Color::rgba(140, 140, 140, 255)),
                        click_change: CheckboxChangeType::ColorChange(Color::rgba(70, 70, 70, 255)),
                    }),
                CheckType::SetRect(
                    CheckRect {
                        rect_color: Color::rgba(200, 200, 200, 255),
                        got_border: false,
                        border_color: Color::rgba(255, 255, 255, 255),
                        border_radius: 2.0,
                        pos: Vec3::new(5.0, 5.0, MENU_WINDOW_CONTENT_DETAIL),
                        size: Vec2::new(14.0, 14.0),
                    }),
                Vec3::new(pos.x + 116.0, pos.y + 92.0, MENU_WINDOW_CONTENT),
                Vec2::new(24.0, 24.0),
                0,
                Some(CheckboxText {
                    text: "Remember account?".to_string(),
                    offset_pos: Vec2::new(3.0, 2.0),
                    render_layer: 1,
                    label_size: Vec2::new(180.0, 20.0),
                    color: Color::rgba(80, 80, 80, 255),
                    hover_change: CheckboxChangeType::ColorChange(Color::rgba(220, 220, 220, 255)),
                    click_change: CheckboxChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                }));
            content.checkbox.push(checkbox);
        }
        WindowType::Register => {
            let size = Vec2::new(348.0, 375.0);
            let pos = Vec2::new((screen_size.x - size.x) * 0.5, 20.0);

            let mut menu_rect = Rect::new(&mut systems.renderer, 0);
            menu_rect.set_position(Vec3::new(pos.x - 1.0, pos.y - 1.0, MENU_WINDOW))
                .set_size(size + 2.0)
                .set_color(Color::rgba(160, 160, 160, 255))
                .set_border_color(Color::rgba(10, 10, 10, 255))
                .set_border_width(1.0);
            content.window.push(systems.gfx.add_rect(menu_rect, 0));
            
            let mut header_rect = Rect::new(&mut systems.renderer, 0);
            header_rect.set_position(Vec3::new(pos.x, pos.y + 345.0, MENU_WINDOW_CONTENT))
                .set_size(Vec2::new(size.x, 30.0))
                .set_color(Color::rgba(120, 120, 120, 255));
            content.window.push(systems.gfx.add_rect(header_rect, 0));

            let header_text = create_label(systems, 
                Vec3::new(pos.x, pos.y + 348.0, MENU_WINDOW_CONTENT_DETAIL), 
                Vec2::new(size.x, 20.0),
                Bounds::new(pos.x, pos.y + 348.0, pos.x + size.x, pos.y + 368.0),
                Color::rgba(200, 200, 200, 255));
            let text_index = systems.gfx.add_text(header_text, 1);
            systems.gfx.set_text(&mut systems.renderer, text_index, "Register Window");
            systems.gfx.center_text(text_index);
            content.label.push(text_index);

            for index in 0..5 {
                let mut labelbox = Rect::new(&mut systems.renderer, 0);
                let mut textbox_bg = Rect::new(&mut systems.renderer, 0);
                let addy = match index {
                    1 => 278.0,
                    2 => 247.0,
                    3 => 222.0,
                    4 => 191.0,
                    _ => 303.0,
                };
                labelbox.set_position(Vec3::new(pos.x + 24.0, pos.y + addy, MENU_WINDOW_CONTENT))
                    .set_size(Vec2::new(116.0, 24.0))
                    .set_color(Color::rgba(208, 208, 208, 255));
                textbox_bg.set_position(Vec3::new(pos.x + 140.0, pos.y + addy, MENU_WINDOW_CONTENT))
                    .set_size(Vec2::new(184.0, 24.0))
                    .set_color(Color::rgba(90, 90, 90, 255));
                content.window.push(systems.gfx.add_rect(labelbox, 0));
                content.window.push(systems.gfx.add_rect(textbox_bg, 0));

                let tpos = Vec2::new(pos.x + 27.0, pos.y + addy + 2.0);
                let text = create_label(systems,
                    Vec3::new(tpos.x, tpos.y, MENU_WINDOW_CONTENT_DETAIL),
                    Vec2::new(110.0, 20.0),
                    Bounds::new(tpos.x, tpos.y, tpos.x + 110.0, tpos.y + 20.0),
                    Color::rgba(100, 100, 100, 255));
                let textindex = systems.gfx.add_text(text, 1);
                let msg = match index {
                    1 => "Retype",
                    2 => "Password",
                    3 => "Retype",
                    4 => "Username",
                    _ => "Email",
                };
                systems.gfx.set_text(&mut systems.renderer, textindex, msg);
                content.label.push(textindex);

                let textbox = Textbox::new(systems,
                    Vec3::new(pos.x + 142.0, pos.y + addy + 2.0, MENU_WINDOW_CONTENT_DETAIL),
                    Vec2::new(180.0, 20.0),
                    Color::rgba(200, 200, 200, 255),
                    1,
                    255,
                    Color::rgba(120, 120, 120, 255),
                    false);
                content.textbox.push(textbox);
            }

            let mut sprite_bg = Rect::new(&mut systems.renderer, 0);
            sprite_bg.set_position(Vec3::new(pos.x + 34.0, pos.y + 98.0, MENU_WINDOW_CONTENT))
                .set_size(Vec2::new(80.0, 80.0))
                .set_color(Color::rgba(120, 120, 120, 255));
            content.window.push(systems.gfx.add_rect(sprite_bg, 0));

            let sprite_label = create_label(systems, 
                Vec3::new(pos.x + 142.0, pos.y + 148.0, MENU_WINDOW_CONTENT_DETAIL), 
                Vec2::new(size.x, 20.0),
                Bounds::new(pos.x + 142.0, pos.y + 148.0, pos.x + 306.0, pos.y + 168.0),
                Color::rgba(80, 80, 80, 255));
            let sprite_index = systems.gfx.add_text(sprite_label, 1);
            systems.gfx.set_text(&mut systems.renderer, sprite_index, "Sprite Selection");
            systems.gfx.center_text(sprite_index);
            content.label.push(sprite_index);

            let button = Button::new(systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(70, 70, 70, 255),
                        border_radius: 0.0,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(180, 180, 180, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(40, 40, 40, 255)),
                    }),
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Register".to_string(),
                        pos: Vec3::new(0.0, 7.0, MENU_WINDOW_CONTENT),
                        color: Color::rgba(200, 200, 200, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(170, 170, 170, 255))
                    }),
                Vec3::new(pos.x + 104.0, pos.y + 45.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 34.0),
                0);
            content.button.push(button);

            let button = Button::new(systems,
                ButtonType::None,
                ButtonContentType::Text(
                    ButtonContentText {
                        text: "Sign In".to_string(),
                        pos: Vec3::new(0.0, 0.0, MENU_WINDOW_CONTENT),
                        color: Color::rgba(80, 80, 80, 255),
                        render_layer: 1,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(220, 220, 220, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(80, 80, 80, 255)),
                    }),
                Vec3::new(pos.x + 104.0, pos.y + 19.0, MENU_WINDOW_CONTENT),
                Vec2::new(140.0, 20.0),
                0);
            content.button.push(button);

            let button = Button::new(systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(70, 70, 70, 255),
                        border_radius: 0.0,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(180, 180, 180, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(40, 40, 40, 255)),
                    }),
                ButtonContentType::Image(
                    ButtonContentImg {
                        res: systems.resource.selection_arrow.allocation,
                        pos: Vec3::new(0.0, 0.0, MENU_WINDOW_CONTENT_DETAIL),
                        uv: Vec2::new(0.0, 0.0),
                        size: Vec2::new(24.0, 24.0),
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None
                    }),
                Vec3::new(pos.x + 142.0, pos.y + 118.0, MENU_WINDOW_CONTENT),
                Vec2::new(24.0, 24.0),
                0);
            content.button.push(button);

            let button = Button::new(systems,
                ButtonType::Rect(
                    ButtonRect {
                        rect_color: Color::rgba(100, 100, 100, 255),
                        got_border: true,
                        border_color: Color::rgba(70, 70, 70, 255),
                        border_radius: 0.0,
                        hover_change: ButtonChangeType::ColorChange(Color::rgba(180, 180, 180, 255)),
                        click_change: ButtonChangeType::ColorChange(Color::rgba(40, 40, 40, 255)),
                    }),
                ButtonContentType::Image(
                    ButtonContentImg {
                        res: systems.resource.selection_arrow.allocation,
                        pos: Vec3::new(0.0, 0.0, MENU_WINDOW_CONTENT_DETAIL),
                        uv: Vec2::new(24.0, 0.0),
                        size: Vec2::new(24.0, 24.0),
                        hover_change: ButtonChangeType::None,
                        click_change: ButtonChangeType::None
                    }),
                Vec3::new(pos.x + 282.0, pos.y + 118.0, MENU_WINDOW_CONTENT),
                Vec2::new(24.0, 24.0),
                0);
            content.button.push(button);

            let sprite_number_text = create_label(systems, 
                Vec3::new(pos.x + 170.0, pos.y + 120.0, MENU_WINDOW_CONTENT_DETAIL), 
                Vec2::new(size.x, 20.0),
                Bounds::new(pos.x + 170.0, pos.y + 120.0, pos.x + 278.0, pos.y + 140.0),
                Color::rgba(80, 80, 80, 255));
            let sprite_number = systems.gfx.add_text(sprite_number_text, 1);
            systems.gfx.set_text(&mut systems.renderer, sprite_number, "0");
            systems.gfx.center_text(sprite_number);
            content.unique_label.push(sprite_number);
        }
        _ => {}
    }
}