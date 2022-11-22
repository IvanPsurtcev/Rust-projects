type IoErr = std::io::Error;
type ParseIntErr = std::num::ParseIntError;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl From<IoErr> for Error {
    fn from(message: IoErr) -> Self {
        Self {
            message: message.to_string(),
        }
    } 
}

impl From<ParseIntErr> for Error {
    fn from(message: ParseIntErr) -> Self {
        Self {
            message: message.to_string(),
        }
    } 
}