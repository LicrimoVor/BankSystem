use image::{RgbaImage, imageops};
use serde::Deserialize;
use std::ffi::{CStr, c_char, c_int};

#[derive(Deserialize)]
struct Params {
    sigma: f32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) -> c_int {
    let params = unsafe {
        let Ok(params) = CStr::from_ptr(params).to_str() else {
            return 1;
        };
        params
    };
    let Ok(Params { sigma }) = serde_json::from_str(params) else {
        return 1;
    };

    let data_vec =
        unsafe { std::slice::from_raw_parts_mut(rgba_data, (width * height * 4) as usize) };
    let vec = data_vec.to_vec();
    let Some(img) = RgbaImage::from_vec(width, height, vec) else {
        return 1;
    };

    let new_img = imageops::blur(&img, sigma);
    let new_vec = new_img.to_vec();
    unsafe {
        std::slice::from_raw_parts_mut(rgba_data, (width * height * 4) as usize)
            .copy_from_slice(&new_vec)
    }
    0
}
