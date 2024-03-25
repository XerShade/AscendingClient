use graphics::*;

use crate::{
    is_within_area, logic::*, socket::sends::*, values::*, widget::*,
    Interface, Item, Socket, SystemHolder,
};

const MAX_STORAGE_X: f32 = 10.0;

#[derive(Clone, Copy, Default)]
struct ItemSlot {
    got_data: bool,
    got_count: bool,
    image: usize,
    count_bg: usize,
    count: usize,
    count_data: u16,
}

pub struct Storage {
    pub visible: bool,
    bg: usize,
    header: usize,
    header_text: usize,
    slot: [usize; MAX_STORAGE],
    item_slot: [ItemSlot; MAX_STORAGE],

    pub pos: Vec2,
    pub size: Vec2,
    pub z_order: f32,
    order_index: usize,
    in_hold: bool,
    hold_pos: Vec2,
    header_pos: Vec2,
    header_size: Vec2,

    min_bound: Vec2,
    max_bound: Vec2,

    pub hold_slot: Option<usize>,
    pub hold_adjust_pos: Vec2,
}

impl Storage {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let w_size = Vec2::new(385.0, 304.0);
        let w_pos = Vec3::new(
            systems.size.width - w_size.x - 10.0,
            60.0,
            ORDER_GUI_WINDOW,
        );
        let pos = Vec2::new(w_pos.x, w_pos.y);

        let detail_1 = w_pos.z.sub_f32(0.001, 3);
        let detail_2 = w_pos.z.sub_f32(0.002, 3);

        let mut rect = Rect::new(&mut systems.renderer, 0);
        rect.set_position(Vec3::new(pos.x - 1.0, pos.y - 1.0, w_pos.z))
            .set_size(w_size + 2.0)
            .set_color(Color::rgba(110, 110, 110, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255));
        let bg = systems.gfx.add_rect(rect, 0);
        systems.gfx.set_visible(bg, false);

        let mut header_rect = Rect::new(&mut systems.renderer, 0);
        let header_pos = Vec2::new(pos.x, pos.y + 274.0);
        let header_size = Vec2::new(w_size.x, 30.0);
        let header_zpos = detail_1;
        header_rect
            .set_position(Vec3::new(header_pos.x, header_pos.y, header_zpos))
            .set_size(header_size)
            .set_color(Color::rgba(70, 70, 70, 255));
        let header = systems.gfx.add_rect(header_rect, 0);
        systems.gfx.set_visible(header, false);

        let text = create_label(
            systems,
            Vec3::new(pos.x, pos.y + 279.0, detail_2),
            Vec2::new(w_size.x, 20.0),
            Bounds::new(pos.x, pos.y + 279.0, pos.x + w_size.x, pos.y + 299.0),
            Color::rgba(200, 200, 200, 255),
        );
        let header_text = systems.gfx.add_text(text, 1);
        systems
            .gfx
            .set_text(&mut systems.renderer, header_text, "Storage");
        systems.gfx.center_text(header_text);
        systems.gfx.set_visible(header_text, false);

        let mut slot = [0; MAX_STORAGE];
        for (i, slot) in slot.iter_mut().enumerate() {
            let mut box_rect = Rect::new(&mut systems.renderer, 0);
            let frame_pos = Vec2::new(
                i as f32 % MAX_STORAGE_X,
                (i as f32 / MAX_STORAGE_X).floor(),
            );
            box_rect
                .set_position(Vec3::new(
                    w_pos.x + 10.0 + (37.0 * frame_pos.x),
                    w_pos.y + 10.0 + (37.0 * frame_pos.y),
                    detail_1,
                ))
                .set_size(Vec2::new(32.0, 32.0))
                .set_color(Color::rgba(200, 200, 200, 255));
            *slot = systems.gfx.add_rect(box_rect, 0);
            systems.gfx.set_visible(*slot, false);
        }

        Storage {
            visible: false,
            bg,
            header,
            header_text,
            slot,
            item_slot: [ItemSlot::default(); MAX_STORAGE],

            pos,
            size: w_size,
            z_order: 0.0,
            order_index: 0,
            in_hold: false,
            hold_pos: Vec2::new(0.0, 0.0),
            header_pos,
            header_size,

            min_bound: Vec2::new(
                systems.size.width - w_size.x - 1.0,
                systems.size.height - w_size.y - 1.0,
            ),
            max_bound: Vec2::new(1.0, 1.0),

            hold_slot: None,
            hold_adjust_pos: Vec2::new(0.0, 0.0),
        }
    }

    pub fn unload(&self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(self.bg);
        systems.gfx.remove_gfx(self.header);
        systems.gfx.remove_gfx(self.header_text);
        self.slot.iter().for_each(|slot| {
            systems.gfx.remove_gfx(*slot);
        });
        self.item_slot.iter().for_each(|item_slot| {
            if item_slot.got_data {
                systems.gfx.remove_gfx(item_slot.image);
                if item_slot.got_count {
                    systems.gfx.remove_gfx(item_slot.count_bg);
                    systems.gfx.remove_gfx(item_slot.count);
                }
            }
        });
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        self.z_order = 0.0;
        systems.gfx.set_visible(self.bg, visible);
        systems.gfx.set_visible(self.header, visible);
        systems.gfx.set_visible(self.header_text, visible);
        self.slot.iter().for_each(|slot| {
            systems.gfx.set_visible(*slot, visible);
        });
        self.item_slot.iter().for_each(|item_slot| {
            if item_slot.got_data {
                systems.gfx.set_visible(item_slot.image, visible);
                if item_slot.got_count {
                    systems.gfx.set_visible(item_slot.count_bg, visible);
                    systems.gfx.set_visible(item_slot.count, visible);
                }
            }
        });
        self.hold_slot = None;
    }

    pub fn hold_storage_slot(&mut self, slot: usize, screen_pos: Vec2) {
        if self.hold_slot.is_some() {
            return;
        }

        self.hold_slot = Some(slot);

        let frame_pos = Vec2::new(
            slot as f32 % MAX_STORAGE_X,
            (slot as f32 / MAX_STORAGE_X).floor(),
        );
        let slot_pos = Vec2::new(
            self.pos.x + 16.0 + (37.0 * frame_pos.x),
            self.pos.y + 195.0 - (37.0 * frame_pos.y),
        );

        self.hold_adjust_pos = screen_pos - slot_pos;
    }

    pub fn move_storage_slot(
        &mut self,
        systems: &mut SystemHolder,
        slot: usize,
        screen_pos: Vec2,
    ) {
        if slot >= MAX_STORAGE || !self.item_slot[slot].got_data {
            return;
        }

        systems.gfx.set_pos(
            self.item_slot[slot].image,
            Vec3::new(
                screen_pos.x - self.hold_adjust_pos.x,
                screen_pos.y - self.hold_adjust_pos.x,
                ORDER_HOLD_ITEM,
            ),
        );

        if self.item_slot[slot].got_count {
            systems.gfx.set_visible(self.item_slot[slot].count, false);
            systems
                .gfx
                .set_visible(self.item_slot[slot].count_bg, false);
        }
    }

    pub fn update_storage_slot(
        &mut self,
        systems: &mut SystemHolder,
        slot: usize,
        data: &Item,
    ) {
        if slot >= MAX_STORAGE {
            return;
        }

        if self.item_slot[slot].got_data {
            systems.gfx.remove_gfx(self.item_slot[slot].image);
            if self.item_slot[slot].got_count {
                systems.gfx.remove_gfx(self.item_slot[slot].count_bg);
                systems.gfx.remove_gfx(self.item_slot[slot].count);
            }
            self.item_slot[slot].got_data = false;
            self.item_slot[slot].got_count = false;
        }

        if data.val == 0 {
            return;
        }

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let item_zpos = detail_origin.sub_f32(0.002, 3);
        let textbg_zpos = detail_origin.sub_f32(0.003, 3);
        let text_zpos = detail_origin.sub_f32(0.004, 3);

        let frame_pos = Vec2::new(
            slot as f32 % MAX_STORAGE_X,
            (slot as f32 / MAX_STORAGE_X).floor(),
        );
        let slot_pos = Vec2::new(
            self.pos.x + 10.0 + (37.0 * frame_pos.x),
            self.pos.y + 195.0 - (37.0 * frame_pos.y),
        );

        let sprite =
            if let Some(itemdata) = systems.base.item.get(data.num as usize) {
                itemdata.sprite as usize
            } else {
                0
            };

        let mut image = Image::new(
            Some(systems.resource.items[sprite].allocation),
            &mut systems.renderer,
            0,
        );
        image.hw = Vec2::new(20.0, 20.0);
        image.uv = Vec4::new(0.0, 0.0, 20.0, 20.0);
        image.pos = Vec3::new(slot_pos.x + 6.0, slot_pos.y + 6.0, item_zpos);
        let image_index = systems.gfx.add_image(image, 0);
        systems.gfx.set_visible(image_index, self.visible);

        self.item_slot[slot].image = image_index;
        self.item_slot[slot].count_data = data.val;

        if data.val > 1 {
            let mut text_bg = Rect::new(&mut systems.renderer, 0);
            text_bg
                .set_size(Vec2::new(32.0, 16.0))
                .set_position(Vec3::new(slot_pos.x, slot_pos.y, textbg_zpos))
                .set_color(Color::rgba(20, 20, 20, 120))
                .set_border_width(1.0)
                .set_border_color(Color::rgba(50, 50, 50, 180));
            let text_bg_index = systems.gfx.add_rect(text_bg, 0);
            systems.gfx.set_visible(text_bg_index, self.visible);

            let text_size = Vec2::new(32.0, 16.0);
            let text = create_label(
                systems,
                Vec3::new(slot_pos.x + 2.0, slot_pos.y + 2.0, text_zpos),
                text_size,
                Bounds::new(
                    slot_pos.x,
                    slot_pos.y,
                    slot_pos.x + text_size.x,
                    slot_pos.y + text_size.y,
                ),
                Color::rgba(240, 240, 240, 255),
            );
            let text_index = systems.gfx.add_text(text, 1);
            systems.gfx.set_text(
                &mut systems.renderer,
                text_index,
                &format!("{}", data.val),
            );
            systems.gfx.set_visible(text_index, self.visible);

            self.item_slot[slot].count = text_index;
            self.item_slot[slot].count_bg = text_bg_index;
        }

        self.item_slot[slot].got_count = data.val > 1;
        self.item_slot[slot].got_data = true;
    }

    pub fn can_hold(&mut self, screen_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }
        is_within_area(screen_pos, self.header_pos, self.header_size)
    }

    pub fn find_storage_slot(
        &mut self,
        screen_pos: Vec2,
        check_empty: bool,
    ) -> Option<usize> {
        for slot in 0..MAX_STORAGE {
            let can_proceed = if self.item_slot[slot].got_data {
                true
            } else {
                check_empty
            };
            if can_proceed {
                let frame_pos = Vec2::new(
                    slot as f32 % MAX_STORAGE_X,
                    (slot as f32 / MAX_STORAGE_X).floor(),
                );
                let slot_pos = Vec2::new(
                    self.pos.x + 10.0 + (37.0 * frame_pos.x),
                    self.pos.y + 195.0 - (37.0 * frame_pos.y),
                );

                if screen_pos.x >= slot_pos.x
                    && screen_pos.x <= slot_pos.x + 32.0
                    && screen_pos.y >= slot_pos.y
                    && screen_pos.y <= slot_pos.y + 32.0
                {
                    return Some(slot);
                }
            }
        }
        None
    }

    pub fn in_window(&mut self, screen_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }
        is_within_area(screen_pos, self.pos, self.size)
    }

    pub fn hold_window(&mut self, screen_pos: Vec2) {
        if self.in_hold {
            return;
        }
        self.in_hold = true;
        self.hold_pos = screen_pos - self.pos;
    }

    pub fn release_window(&mut self) {
        self.in_hold = false;
    }

    pub fn set_z_order(
        &mut self,
        systems: &mut SystemHolder,
        z_order: f32,
        order_index: usize,
    ) {
        if self.z_order == z_order {
            return;
        }
        self.z_order = z_order;
        self.order_index = order_index;

        let detail_origin = ORDER_GUI_WINDOW.sub_f32(self.z_order, 3);
        let detail_1 = detail_origin.sub_f32(0.001, 3);
        let detail_2 = detail_origin.sub_f32(0.002, 3);
        let detail_3 = detail_origin.sub_f32(0.003, 3);
        let detail_4 = detail_origin.sub_f32(0.004, 3);

        let mut pos = systems.gfx.get_pos(self.bg);
        pos.z = detail_origin;
        systems.gfx.set_pos(self.bg, pos);

        let mut pos = systems.gfx.get_pos(self.header);
        let header_zpos = detail_1;
        pos.z = header_zpos;
        systems.gfx.set_pos(self.header, pos);

        let mut pos = systems.gfx.get_pos(self.header_text);
        pos.z = detail_2;
        systems.gfx.set_pos(self.header_text, pos);

        for i in 0..MAX_STORAGE {
            let mut pos = systems.gfx.get_pos(self.slot[i]);
            pos.z = detail_1;
            systems.gfx.set_pos(self.slot[i], pos);

            let can_proceed = if let Some(hold_slot) = self.hold_slot {
                hold_slot != i
            } else {
                true
            };
            if self.item_slot[i].got_data && can_proceed {
                let mut pos = systems.gfx.get_pos(self.item_slot[i].image);
                pos.z = detail_2;
                systems.gfx.set_pos(self.item_slot[i].image, pos);
            }

            if self.item_slot[i].got_count {
                let mut pos = systems.gfx.get_pos(self.item_slot[i].count_bg);
                pos.z = detail_3;
                systems.gfx.set_pos(self.item_slot[i].count_bg, pos);

                let mut pos = systems.gfx.get_pos(self.item_slot[i].count);
                pos.z = detail_4;
                systems.gfx.set_pos(self.item_slot[i].count, pos);
            }
        }
    }

    pub fn move_window(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        if !self.in_hold {
            return;
        }
        self.pos = (screen_pos - self.hold_pos)
            .max(self.max_bound)
            .min(self.min_bound);

        let pos = systems.gfx.get_pos(self.bg);
        systems.gfx.set_pos(
            self.bg,
            Vec3::new(self.pos.x - 1.0, self.pos.y - 1.0, pos.z),
        );
        let pos = systems.gfx.get_pos(self.header);
        self.header_pos = Vec2::new(self.pos.x, self.pos.y + 274.0);
        systems.gfx.set_pos(
            self.header,
            Vec3::new(self.pos.x, self.pos.y + 274.0, pos.z),
        );
        let pos = systems.gfx.get_pos(self.header_text);
        systems.gfx.set_pos(
            self.header_text,
            Vec3::new(self.pos.x, self.pos.y + 279.0, pos.z),
        );
        systems.gfx.set_bound(
            self.header_text,
            Bounds::new(
                self.pos.x,
                self.pos.y + 279.0,
                self.pos.x + self.size.x,
                self.pos.y + 299.0,
            ),
        );
        systems.gfx.center_text(self.header_text);

        let item_text_size = Vec2::new(32.0, 16.0);
        for i in 0..MAX_STORAGE {
            let frame_pos = Vec2::new(
                i as f32 % MAX_STORAGE_X,
                (i as f32 / MAX_STORAGE_X).floor(),
            );
            let slot_pos = Vec2::new(
                self.pos.x + 10.0 + (37.0 * frame_pos.x),
                self.pos.y + 232.0 - (37.0 * frame_pos.y),
            );

            let pos = systems.gfx.get_pos(self.slot[i]);
            systems.gfx.set_pos(
                self.slot[i],
                Vec3::new(slot_pos.x, slot_pos.y, pos.z),
            );

            if self.item_slot[i].got_data {
                let pos = systems.gfx.get_pos(self.item_slot[i].image);
                systems.gfx.set_pos(
                    self.item_slot[i].image,
                    Vec3::new(slot_pos.x + 6.0, slot_pos.y + 6.0, pos.z),
                );

                if self.item_slot[i].got_count {
                    let pos = systems.gfx.get_pos(self.item_slot[i].count_bg);
                    systems.gfx.set_pos(
                        self.item_slot[i].count_bg,
                        Vec3::new(slot_pos.x, slot_pos.y, pos.z),
                    );

                    let pos = systems.gfx.get_pos(self.item_slot[i].count);
                    systems.gfx.set_pos(
                        self.item_slot[i].count,
                        Vec3::new(slot_pos.x + 2.0, slot_pos.y + 2.0, pos.z),
                    );
                    systems.gfx.set_bound(
                        self.item_slot[i].count,
                        Bounds::new(
                            slot_pos.x,
                            slot_pos.y,
                            slot_pos.x + item_text_size.x,
                            slot_pos.y + item_text_size.y,
                        ),
                    );
                }
            }
        }
    }
}
