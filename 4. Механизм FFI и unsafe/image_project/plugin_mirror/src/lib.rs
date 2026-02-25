use image::{RgbaImage, imageops};
use std::ffi::{c_char, c_int};

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
    _: *const c_char,
) -> c_int {
    if rgba_data.is_null() {
        return 1;
    }
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

    let new_img = imageops::flip_horizontal(&img);
    let new_vec = new_img.to_vec();
    unsafe { std::slice::from_raw_parts_mut(rgba_data, length as usize).copy_from_slice(&new_vec) }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_image_mirror_valid() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mirror_data = vec![5, 6, 7, 8, 1, 2, 3, 4];
        let mut data_clone = data.clone();
        let data_ptr = data_clone.as_mut_ptr();
        let params = CString::new("{}").unwrap();
        let params_ptr = params.as_ptr();

        let result = unsafe { process_image(2, 1, data_ptr, params_ptr) };
        assert_eq!(result, 0);
        assert_eq!(data_clone, mirror_data);
    }

    #[test]
    fn test_image_mirror_invalid() {
        let result = unsafe { process_image(2, 1, std::ptr::null_mut(), std::ptr::null()) };
        assert_eq!(result, 1);
    }
}
