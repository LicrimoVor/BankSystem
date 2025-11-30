#[derive(Debug)]
pub enum ParseFromFileError {
    IoError(std::io::Error),

    ParseError(&'static str),
}
