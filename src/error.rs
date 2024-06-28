pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoParentDirectory,
    Io(std::io::Error),
    Notify(notify_debouncer_mini::notify::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<notify_debouncer_mini::notify::Error> for Error {
    fn from(value: notify_debouncer_mini::notify::Error) -> Self {
        Self::Notify(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
