use clap::Parser;
#[derive(Parser)]
#[command(name = "processor-cli")]
pub struct Cli {
    #[arg(short, long, help = "Путь к исходному PNG-изображению")]
    pub input: String,

    #[arg(
        short,
        long,
        help = "путь, по которому будет сохранено обработанное изображение."
    )]
    pub output: String,

    #[arg(
        long,
        help = "имя плагина (динамической библиотеки) без расширения (например, invert)."
    )]
    pub plugin: String,

    #[arg(short, long, help = "путь к текстовому файлу с параметрами обработки.")]
    pub params: String,

    #[arg(
        long,
        help = "путь к директории, где находится плагин (по умолчанию target/debug)."
    )]
    pub plugin_path: Option<String>,
}
