use std::ffi::{CStr, c_char, c_int};

// repr(C) говорит компилятору, что декларация структуры должна
// происходить по таким же принципам, как и в C, а именно без перестановок
// полей в памяти (компилятор Rust по умолчанию может менять порядок
// полей по своему усмотрению, к примеру, чтобы оптимизировать размер структуры
#[repr(C)]
pub struct Cased {
    cstring: *const c_char,
    case: bool, // true for uppercase,
                // false for lowercase
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn count_case_ascii(c: Cased) -> u32 {
    let cstring = unsafe { CStr::from_ptr(c.cstring) };
    let case = !c.case;
    let mut counter = 0;

    for c in cstring.to_bytes() {
        println!("{c}");
        if !(((*c >= 65) && (*c <= 90)) || ((*c >= 97) && (*c <= 122))) {
            continue;
        }
        if (case && *c >= 97) || (!case && *c <= 90) {
            counter += 1;
        }
    }

    counter
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn doublefast(v: c_int) -> c_int {
    v.pow(2)
}
