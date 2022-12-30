use std::collections::HashMap;
use winit::event::VirtualKeyCode;

macro_rules! keymap {
    ($($keycode:ident => $mapping:literal),*) => {
        lazy_static::lazy_static! {
            pub static ref KEYMAP: HashMap<VirtualKeyCode, u8> = {
                let mut m = HashMap::new();
                $(
                  m.insert(VirtualKeyCode::$keycode, $mapping);
                )*
                m
            };
        }
    };
}

keymap! {
    Key1 => 0x1,
    Key2 => 0x2,
    Key3 => 0x3,
    Key4 => 0xC,
    Q => 0x4,
    W => 0x5,
    E => 0x6,
    R => 0xD,
    A => 0x7,
    S => 0x8,
    D => 0x9,
    F => 0xE,
    Z => 0xA,
    X => 0x0,
    C => 0xB,
    V => 0xF
}
