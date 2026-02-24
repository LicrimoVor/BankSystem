use image::{RgbaImage, imageops};
use serde::Deserialize;
use std::ffi::{CStr, c_char, c_int};

#[derive(Deserialize)]
struct Params {
    sigma: f32,
}

/// Processes the image by applying horizontal reflection.
///
/// # Parameters
/// * `width' - The width of the image in pixels
/// * `height` - Image height in pixels
/// * `rgba_data' - Pointer to an array of RGBA image data (size: width * height * 4)
/// * `_` - Parameter for compatibility with the plug-in interface (not used)
///
/// # Returns
/// * `0` - Successful processing
/// * `1` - Error when creating an image from data
////
/// # SAFETY:
/// - The `rgba_data` pointer is valid and indicates a sufficient amount of memory
/// - The data size corresponds to width * height * 4 bytes
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) -> c_int {
    if params.is_null() || rgba_data.is_null() {
        return 1;
    }
    let params = unsafe {
        let Ok(params) = CStr::from_ptr(params).to_str() else {
            return 1;
        };
        params
    };
    let Ok(Params { sigma }) = serde_json::from_str(params) else {
        return 1;
    };
    let Some(length) = width
        .checked_mul(height)
        .and_then(|l| u32::checked_mul(l, 4))
    else {
        return 1;
    };

    let data_vec = unsafe { std::slice::from_raw_parts_mut(rgba_data, length as usize) };
    let vec = data_vec.to_vec();
    let Some(img) = RgbaImage::from_vec(width, height, vec) else {
        return 1;
    };

    let new_img = imageops::blur(&img, sigma);
    let new_vec = new_img.to_vec();
    unsafe { std::slice::from_raw_parts_mut(rgba_data, length as usize).copy_from_slice(&new_vec) }
    0
}
