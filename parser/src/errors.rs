#[derive(Debug)]
pub enum ParseFileError {
    IoError(std::io::Error),

    ParseError(&'static str),

    WriteError(std::io::Error),
}
