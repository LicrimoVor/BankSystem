use anyhow::{Result, anyhow};
use libloading::Symbol;
use std::{
    ffi::{CString, c_char, c_int},
    panic::{AssertUnwindSafe, catch_unwind},
};
use tracing::info;

#[allow(non_camel_case_types)]
type process_image<'a> = Symbol<
    'a,
    unsafe extern "C-unwind" fn(
        width: u32,
        height: u32,
        rgba_data: *mut u8,
        params: *const c_char,
    ) -> c_int,
>;

pub struct PluginInterface<'a> {
    inner_process_image: process_image<'a>,
}

impl<'a> PluginInterface<'a> {
    /// Creates a new `PluginInterface` instance by loading the `process_image` function from the given plugin.
    ///
    /// # Arguments
    /// * `plugin` - A reference to the `Plugin` instance containing the loaded dynamic library.
    ///
    /// # Returns
    /// A `Result` containing the `PluginInterface` if successful, or an error if the `process_image` symbol cannot be loaded.
    ///
    /// # Safety Invariants:
    /// - The `process_image` symbol must exist in the dynamic library and have the correct signature.
    /// - The dynamic library must remain valid for the lifetime of the `PluginInterface`.
    pub fn new(plugin: &'a Plugin) -> Result<Self> {
        info!("Загружаем плагин");
        let process_image = unsafe { plugin.lib.get("process_image")? };
        info!("Плагин загружен");

        Ok(PluginInterface {
            inner_process_image: process_image,
        })
    }

    /// Processes an image using the dynamically loaded `process_image` function.
    ///
    /// # Arguments
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    /// * `rgba_data` - A mutable reference to the RGBA pixel data of the image.
    /// * `params` - A JSON string containing parameters for the image processing.
    ///
    /// # Returns
    /// A `Result` indicating success if the plugin returns `0`, or an error if the plugin returns a non-zero value.
    ///
    /// # SAFETY:
    /// - `rgba_data` must be large enough to hold `width * height * 4` bytes.
    /// - The `params` string must be valid UTF-8 and properly null-terminated when converted to a C string.
    /// - The `process_image` function must not retain references to `rgba_data` or `params` after it returns.
    pub fn process_image(
        &self,
        width: u32,
        height: u32,
        rgba_data: &mut Vec<u8>,
        params: String,
    ) -> Result<()> {
        let ptr = CString::new(params)?;
        // проверку перенес на ХОСТ (думаю он должен гаранировать корректность вызова)
        let Some(length) = width
            .checked_mul(height)
            .and_then(|l| u32::checked_mul(l, 4))
        else {
            return Err(anyhow!("Invalid rgba_data size"));
        };

        if rgba_data.len() != length as usize {
            return Err(anyhow!("Invalid rgba_data size"));
        }

        let rgba_data = rgba_data.as_mut_ptr();
        // пытался я перехватить, но не получилось и ошибка прекращает процесс
        let result = catch_unwind(AssertUnwindSafe(|| unsafe {
            (self.inner_process_image)(width, height, rgba_data, ptr.as_ptr())
        }));

        match result {
            Ok(0) => Ok(()),
            Ok(_) => Err(anyhow!("Plugin error")),
            Err(_) => Err(anyhow!("Plugin panic")),
        }
    }
}

/// Represents a dynamically loaded library (plugin).
pub struct Plugin {
    lib: libloading::Library,
}

impl Plugin {
    /// Creates a new `Plugin` instance by loading the dynamic library at the specified path.
    ///
    /// # Arguments
    /// * `path` - The file system path to the dynamic library.
    ///
    /// # Returns
    /// A `Result` containing the `Plugin` if successful, or an error if the library cannot be loaded.
    ///
    /// # SAFETY:
    /// - The path must point to a valid dynamic library file.
    /// - The library must remain accessible and unchanged for the lifetime of the `Plugin`.
    pub fn new(path: &str) -> Result<Self> {
        let lib = unsafe { libloading::Library::new(path)? };
        Ok(Plugin { lib })
    }
}
