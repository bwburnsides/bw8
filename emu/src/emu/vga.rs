use itertools::iproduct;

use super::bus::Bw8Bus;

pub const FRAMERATE: f64 = 60.0;

pub const COLUMN_COUNT: usize = 640;
pub const ROW_COUNT: usize = 480;

#[repr(C)]
#[repr(align(4))]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    channels: [arch::Byte; 4],
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);

    pub const fn new(red: arch::Byte, green: arch::Byte, blue: arch::Byte) -> Self {
        Self {
            channels: [red, green, blue, u8::MAX],
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        Self::new(
            ((value >> 5) & 0b111) * 32,
            ((value >> 2) & 0b111) * 32,
            ((value >> 0) & 0b11) * 64,
        )
    }
}

pub struct Vga {
    pixels: Box<[Color; COLUMN_COUNT * ROW_COUNT]>,
}

impl Vga {
    pub fn new() -> Self {
        Self {
            pixels: Box::new([Color::BLACK; COLUMN_COUNT * ROW_COUNT]),
        }
    }

    pub fn pixel_data<'a>(&'a self) -> &'a [u8] {
        const COLOR_SIZE: usize = std::mem::size_of::<Color>();

        // TODO: Don't use unsafe block, lol
        unsafe {
            let pixel_ptr = self.pixels.as_ptr();
            let data_ptr = pixel_ptr as *const u8;
            let len = self.pixels.len() * COLOR_SIZE;
            std::slice::from_raw_parts(data_ptr, len)
        }
    }

    pub fn reset(&mut self) {
        self.pixels = Box::new([Color::BLACK; COLUMN_COUNT * ROW_COUNT]);
    }

    pub fn clock(&mut self, bus: &Bw8Bus) {
        // (C,   R)
        // (0,   0) at 0x8000,1
        // (1,   0) at 0x8002,3
        // (2,   0) at 0x8004,5
        // (3,   0) at 0x8006,7
        // .....
        // (79,  0) at 0x80F2,3
        // (0,   1) at 0x8100,1
        // (1,   1) at 0x8102,3

        // Bitmap is laid out by row, then column, then color plane
        // So:
        // 0: <Row0, Plane0>
        // 1: <Row1, Plane0>
        // 2: <Row2, Plane0>
        // ...
        // 7: <Row7, Plane0>
        // 8: <Row0, Plane1>
        // ...
        // 15: <Row7, Plane1>
        // 16: <Row0, Plane2>
        // ...
        // 23: <Row7, Plane2>
        // 24: <Row0, Plane3>
        // ...
        // 31: <Row7, Plane3>

        const TILEMAP_OFFSET: arch::Address = 0x0000;
        const BITMAP_OFFSET: arch::Address = 0x4000;
        const PALETTE_OFFSET: arch::Address = 0x6000;

        // 0 -> 7    7 - 0 = 7
        // 1 -> 6    7 - 1 = 6
        // 2 -> 5
        // 3 -> 4
        // 4 -> 3
        // 5 -> 2
        // 6 -> 1
        // 7 -> 0    7 - 7 =  0

        for (x, y) in iproduct!(0..640, 0..480) {
            let tile_x = x / 8;
            let tile_y = y / 8;

            let map_address = TILEMAP_OFFSET + (256 * tile_y) + (2 * tile_x);
            let bitmap_id = bus.inspect_framebuffer(map_address);
            let palette_id = bus.inspect_framebuffer(map_address + 1);

            let bitmap_address = BITMAP_OFFSET + ((32 * bitmap_id) as u16);
            let bitmap_col = x & 0b111;
            let bitmap_row = y & 0b111;

            let plane_0 = bus.inspect_framebuffer(bitmap_address + bitmap_row);
            let plane_1 = bus.inspect_framebuffer(bitmap_address + bitmap_row + 8);
            let plane_2 = bus.inspect_framebuffer(bitmap_address + bitmap_row + 16);
            let plane_3 = bus.inspect_framebuffer(bitmap_address + bitmap_row + 24);

            let palette_index = (((plane_0 >> (7 - bitmap_col)) & 1) << 0)
                | (((plane_1 >> (7 - bitmap_col)) & 1) << 1)
                | (((plane_2 >> (7 - bitmap_col)) & 1) << 2)
                | (((plane_3 >> (7 - bitmap_col)) & 1) << 3);

            let color_address =
                PALETTE_OFFSET + ((16 * palette_id) as u16) + (palette_index as u16);
            let color = bus.inspect_framebuffer(color_address);
            self.pixels[(x as usize) + (y as usize) * COLUMN_COUNT] = Color::from(color);
        }
    }
}
