use crate::vecmath::Color3U8;
use libc::{c_char, c_int, c_void};
use std::ffi::CString;
use std::path::Path;

#[link(name = "stb_image_write")]
extern "C" {
    fn stbi_write_bmp(
        filename: *const c_char,
        w: c_int,
        h: c_int,
        comp: c_int,
        data: *const c_void,
    ) -> c_int;
}

const IMAGE_PIXEL_SIZE: usize = 3;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[u8; IMAGE_PIXEL_SIZE]>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width: width,
            height: height,
            pixels: vec![[0, 0, 0]; (width * height) as usize],
        }
    }

    pub fn write_pixel(&mut self, row: u32, col: u32, pixel: Color3U8) {
        self.pixels[(row * self.width + col) as usize] = pixel.into();
    }

    pub fn write_bmp(&self, filename: &Path) -> Result<(), ()> {
        let filename = filename.to_str().unwrap();
        let filename = CString::new(filename).unwrap();

        let result = unsafe {
            stbi_write_bmp(
                filename.into_raw(),
                self.width as c_int,
                self.height as c_int,
                IMAGE_PIXEL_SIZE as c_int,
                self.pixels.as_ptr() as *const c_void,
            )
        };

        if result != 0 {
            Result::Ok(())
        } else {
            Result::Err(())
        }
    }
}
