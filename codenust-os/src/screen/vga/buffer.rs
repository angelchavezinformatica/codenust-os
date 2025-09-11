use crate::screen::vga::color::ColorCode;
use volatile::Volatile;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

#[allow(dead_code)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
