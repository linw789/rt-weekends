use crate::types::Fp;
use crate::vecmath::{Color3U8, Color3F};
use libc::{c_char, c_int, c_void};
use std::ffi::CString;
use std::path::Path;
use std::mem;

#[link(name = "stb_image")]
extern "C" {
    fn stbi_write_bmp(
        filename: *const c_char,
        w: c_int,
        h: c_int,
        comp: c_int,
        data: *const c_void,
    ) -> c_int;
}

#[link(name = "stb_image")]
extern "C" {
    fn stbi_load(
        filename: *const c_char,
        w: *const c_int,
        h: *const c_int,
        comp_n: *const c_int,
        desire_comp_n: c_int,
    ) -> *const c_char;
}

#[link(name = "stb_image")]
extern "C" {
    fn stbi_image_free(data: *mut c_void);
}

pub const IMAGE_PIXEL_SIZE: usize = 3;

pub struct Image {
    from_file: bool,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[u8; IMAGE_PIXEL_SIZE]>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let size = (width * height) as usize;
        Image {
            from_file: false,
            width,
            height,
            pixels: vec![[0, 0, 0]; size],
        }
    }

    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Image {
        let src_file_path_cstr = CString::new(file_path.as_ref().to_str().unwrap()).unwrap();
        let mut image_width = 0;
        let mut image_height = 0;
        let mut image_components = 0;

        let image_data = unsafe {
            stbi_load(
                src_file_path_cstr.into_raw(),
                &mut image_width,
                &mut image_height,
                &mut image_components,
                0)
        };
        assert!(image_data != std::ptr::null());
        assert!(image_components == IMAGE_PIXEL_SIZE as c_int);

        Image {
            from_file: true,
            width: image_width as u32,
            height: image_height as u32,
            pixels: unsafe {
                let pixel_size: usize = (image_width * image_height).try_into().unwrap();
                Vec::from_raw_parts(
                    image_data as *mut [u8; IMAGE_PIXEL_SIZE],
                    pixel_size,
                    pixel_size)
            },
        }
    }

    pub fn pixel_at_uv(&self, u: Fp, v: Fp) -> Color3F {
        let w = (u * self.width as Fp) as u32;
        let w = if w >= self.width { self.width - 1 } else { w };
        let h = (v * self.height as Fp) as u32;
        let h = if h >= self.height { self.height - 1 } else { h };
        let index = h * self.width + w;
        let pixel: Color3U8 = self.pixels[index as usize].into();
        pixel.into()
    }

    #[allow(dead_code)]
    pub fn write_pixel(&mut self, row: u32, col: u32, pixel: Color3U8) {
        self.pixels[(row * self.width + col) as usize] = pixel.into();
    }

    pub fn write_row(&mut self, row_index: u32, row: &[[u8; IMAGE_PIXEL_SIZE]]) {
        assert!(row_index < self.height);

        let pixel_start = (row_index * self.width) as usize;
        for i in 0..row.len() {
            if (i as u32) >= self.width {
                break;
            }
            self.pixels[pixel_start + i] = row[i];
        }
    }

    pub fn write_bmp(&self, filename: &Path) -> Result<(), ()> {
        let filename = CString::new(filename.to_str().unwrap()).unwrap();
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

impl Drop for Image {
    fn drop(&mut self) {
        if self.from_file {
            let to_drop = mem::replace(&mut self.pixels, Vec::new());
            let image_data = to_drop.leak();
            unsafe {
                stbi_image_free(image_data.as_ptr() as *mut c_void);
            }

        } else {
            let to_drop = mem::replace(&mut self.pixels, Vec::new());
            mem::drop(to_drop);
        }
    }
}
