include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
use std::{
    ffi::{CStr, CString, c_int},
    io::Error,
};

const TEST_JSON: &CStr = c"{
    \"meaning_of_life\": 42
}";

#[unsafe(no_mangle)]
extern "C" fn safe_strerror_s(error: c_int) -> Result<String, Error> {
    let buf: *mut i8 = [0; 256].as_mut_ptr();
    let res: c_int = unsafe { strerror_s(buf, 256, error) };
    if res == 0 {
        Ok(unsafe { CStr::from_ptr(buf) }.to_string_lossy().to_string())
    } else {
        Err(Error::from_raw_os_error(res as i32))
    }
}

fn main() {
    let json: *mut cJSON = unsafe { cJSON_Parse(TEST_JSON.as_ptr()) };

    let json_str = unsafe { cJSON_PrintUnformatted(json) };
    let json_str = unsafe { CString::from_raw(json_str) };
    let json_str = json_str.to_str().unwrap();
    assert_eq!(json_str, r#"{"meaning_of_life":42}"#);

    let meaning_of_life = unsafe { cJSON_GetObjectItem(json, c"meaning_of_life".as_ptr()) };
    let meaning_of_life = unsafe { cJSON_GetNumberValue(meaning_of_life) };
    println!("Meaning of life: {}", meaning_of_life);
    assert_eq!(meaning_of_life, 42f64);

    for i in -128..128 {
        let res = safe_strerror_s(i);
        println!("{:?}", res);
    }
}
