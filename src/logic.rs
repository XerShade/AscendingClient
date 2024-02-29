use graphics::*;
use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub enum InsertTypes {
    Int(i64),
    UInt(u64),
    Str(String),
    Bool(bool),
}

impl InsertTypes {
    pub fn get_int(&self) -> i64 {
        match self {
            InsertTypes::Int(data) => *data,
            _ => 0,
        }
    }

    pub fn get_uint(&self) -> u64 {
        match self {
            InsertTypes::UInt(data) => *data,
            _ => 0,
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            InsertTypes::Str(data) => data.clone(),
            _ => String::new(),
        }
    }
}

pub fn get_screen_center(size: &PhysicalSize<f32>) -> Vec2 {
    Vec2::new((size.width * 0.5).floor(),
        (size.height * 0.5).floor())
}

pub fn is_within_area(area: Vec2, target_pos: Vec2, target_size: Vec2) -> bool {
    area.x >= target_pos.x &&
        area.x <= target_pos.x + target_size.x &&
        area.y >= target_pos.y &&
        area.y <= target_pos.y + target_size.y
}

pub fn next_down(f: f32) -> f32 {
    // We must use strictly integer arithmetic to prevent denormals from
    // flushing to zero after an arithmetic operation on some platforms.
    const NEG_TINY_BITS: u32 = 0x8000_0001; // Smallest (in magnitude) negative f32.
    const CLEAR_SIGN_MASK: u32 = 0x7fff_ffff;

    let bits = f.to_bits();
    if f.is_nan() || bits == f32::NEG_INFINITY.to_bits() {
        return f;
    }

    let abs = bits & CLEAR_SIGN_MASK;
    let next_bits = if abs == 0 {
        NEG_TINY_BITS
    } else if bits == abs {
        bits - 1
    } else {
        bits + 1
    };
    f32::from_bits(next_bits)
}