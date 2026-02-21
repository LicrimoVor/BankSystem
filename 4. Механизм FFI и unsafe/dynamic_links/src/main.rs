use std::ffi::c_int;

fn main() -> anyhow::Result<()> {
    let lib = unsafe { libloading::Library::new("./libmylib.so")? };
    let square: libloading::Symbol<unsafe extern "C" fn(c_int) -> c_int> =
        unsafe { lib.get(b"square")? };

    println!("Squared eleven: {}", unsafe { square(11) });
    Ok(())
}
