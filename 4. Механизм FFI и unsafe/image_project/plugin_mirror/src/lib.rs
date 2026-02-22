use image::{RgbaImage, imageops};
use std::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    _: *const c_char,
) -> c_int {
    let data_vec =
        unsafe { std::slice::from_raw_parts_mut(rgba_data, (width * height * 4) as usize) };
    let vec = data_vec.to_vec();
    let Some(img) = RgbaImage::from_vec(width, height, vec) else {
        return 1;
    };

    let new_img = imageops::flip_horizontal(&img);
    let new_vec = new_img.to_vec();
    unsafe {
        std::slice::from_raw_parts_mut(rgba_data, (width * height * 4) as usize)
            .copy_from_slice(&new_vec)
    }
    0
}
