#![no_std]
#![no_main]
#![allow(arithmetic_overflow)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn from_hex(num: u32) -> Color {
        let b = num as u8;
        let g = (num >> 8) as u8;
        let r = (num >> 16) as u8;
        Color(r, g, b, 0xFF)
    }
    pub fn to_buffer(&mut self, buffer: &mut [u8]) {
        let mut red = self.0;
        let mut green = self.1;
        let mut blue = self.2;
        let mut alpha = self.3;

        buffer[0] = blue & 0xFF;
        buffer[1] = green & 0xFF;
        buffer[2] = red & 0xFF;
        buffer[3] = alpha & 0xFF;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let buffer = framebuffer.buffer_mut();
        let screen_size = 640 * 480;
        let mut color = Color::from_hex(0x000000);
        let mut x = 0;
        let mut y = 0;
        // vertical 480 horizontal 640;
        loop {
            x = 0;
            y = 0;
            color = Color::from_hex(0x000000);
            for i in 0..screen_size {
                color = Color::from_hex(0x0 + i as u32);
                let offset = i * 4;
                color.to_buffer(&mut buffer[offset..(offset + 4)]);

                x += 1;
                if i % 640 == 0 && i != 0 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }
    loop {}
}

entry_point!(kernel_main);
