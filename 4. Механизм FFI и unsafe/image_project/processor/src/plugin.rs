use anyhow::Result;
use libloading::Symbol;
use std::ffi::{CString, c_char, c_int};
use tracing::info;

#[allow(non_camel_case_types)]
type process_image<'a> = Symbol<
    'a,
    unsafe fn(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char) -> c_int,
>;

pub struct PluginInterface<'a> {
    inner_process_image: process_image<'a>,
}

impl<'a> PluginInterface<'a> {
    pub fn new(plugin: &'a Plugin) -> Result<Self> {
        info!("Загружаем плагин");
        let process_image = unsafe { plugin.lib.get("process_image")? };
        info!("Плагин загружен");

        Ok(PluginInterface {
            inner_process_image: process_image,
        })
    }

    pub fn process_image(
        &self,
        width: u32,
        height: u32,
        rgba_data: &mut Vec<u8>,
        params: String,
    ) -> Result<()> {
        let ptr = CString::new(params)?;
        let rgba_data = rgba_data.as_mut_ptr();
        match unsafe { (self.inner_process_image)(width, height, rgba_data, ptr.as_ptr()) } {
            0 => Ok(()),
            _ => Err(anyhow::Error::msg("Plugin error")),
        }
    }
}

pub struct Plugin {
    lib: libloading::Library,
}

impl Plugin {
    pub fn new(path: &str) -> Result<Self> {
        let lib = unsafe { libloading::Library::new(path)? };
        Ok(Plugin { lib })
    }
}
