use crate::plugin::{Plugin, PluginInterface};
use clap::Parser;
use std::{env, fs};
use tracing::{error, info, warn};
mod cli;
mod plugin;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let cli::Cli {
        input,
        output,
        plugin,
        params,
        plugin_path,
    } = cli::Cli::try_parse()?;

    if output.ends_with("jpg") {
        error!("jpg не поддерживается");
        return Err(anyhow::anyhow!("jpg не поддерживается"));
    }

    let root = env::current_dir()?;
    info!("Собираем плагин");
    let plugin = match plugin_path {
        Some(plugin_path) => Plugin::new(&plugin_path),
        None => {
            // сначала ищем в release
            let plugin_path = root.join("target").join("release").join(plugin.clone());
            info!("Путь к плагину: {plugin_path:?}");
            match Plugin::new(plugin_path.to_str().unwrap()) {
                Ok(plugin) => Ok(plugin),
                Err(_) => {
                    warn!("Не получилось найти в release");
                    // потом ищем в debug
                    let plugin_path = root.join("target").join("debug").join(plugin);
                    info!("Путь к плагину: {plugin_path:?}");
                    match Plugin::new(plugin_path.to_str().unwrap()) {
                        Ok(plugin) => Ok(plugin),
                        Err(err) => {
                            warn!("Не получилось найти в debug");
                            return Err(err);
                        }
                    }
                }
            }
        }
    }?;
    info!("Плагин собран");

    let interface = PluginInterface::new(&plugin)?;
    info!("Загружаем параметры");
    let params = fs::read_to_string(root.join(params))?;
    info!("Загружаем изображение");
    let img = image::open(&input)?.to_rgba8();
    let mut data = img.to_vec();
    info!("Обрабатываем изображение");
    interface.process_image(img.width(), img.height(), &mut data, params)?;
    info!("Сохраняем изображение");
    let new_img = image::RgbaImage::from_vec(img.width(), img.height(), data).ok_or(
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid data"),
    )?;
    new_img.save(&output)?;

    info!("Готово");
    Ok(())
}
