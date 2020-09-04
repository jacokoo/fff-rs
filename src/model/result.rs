use crate::ui::event::EventBody;
use crossbeam_channel::SendError;

#[derive(Debug)]
pub enum Error {
    Io(std::io::ErrorKind),

    PathNotExists(String),
    InvalidPath(String),
    InvalidProtocolString(String),
    ProtocolNotSupported(String),

    FileAlreadyExists(String),

    InvalidEnumValue(String),
    InvalidFilter(String),
    DirIsRequired(String),
    SendError(EventBody),
}

pub type Res<T> = std::result::Result<T, Error>;
pub type Void = Res<()>;

pub fn option_from_result<T, E>(r: Result<T, E>) -> Option<T> {
    if let Ok(t) = r {
        Some(t)
    } else {
        None
    }
}

impl Error {
    pub fn res<T>(self) -> Res<T> {
        Result::Err(self)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::Io(e.kind());
    }
}

impl From<crossbeam_channel::SendError<EventBody>> for Error {
    fn from(e: SendError<EventBody>) -> Self {
        Self::SendError(e.0)
    }
}
