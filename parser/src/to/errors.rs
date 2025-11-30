#[derive(Debug)]
pub enum ParseToFileError {
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),

    ParseError(String),
}
