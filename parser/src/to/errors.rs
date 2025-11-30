#[derive(Debug)]
pub enum ParseFromFileError {
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),

    ParseError(String),
}
