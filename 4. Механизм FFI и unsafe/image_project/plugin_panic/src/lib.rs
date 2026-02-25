use std::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(_: u32, _: u32, _: *mut u8, _: *const c_char) -> c_int {
    panic!("Panic in process_image");
}
