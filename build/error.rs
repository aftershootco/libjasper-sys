use std::backtrace::Backtrace;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    backtrace: Backtrace,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("backtrace", &self.backtrace)
            .finish()
    }
}

impl<E: Into<ErrorKind>> From<E> for Error {
    #[track_caller]
    fn from(e: E) -> Self {
        Self {
            kind: e.into(),
            backtrace: Backtrace::capture(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("{0}")]
    VarError(#[from] std::env::VarError),
    #[error("Failed to clone the git repo")]
    CloneError,
    #[error("{0}")]
    Join(#[from] std::env::JoinPathsError),
    #[error("{0:?}")]
    AnyErr(AnyErr),
}

// impl<E: Into<AnyErr>> From<E> for ErrorKind {
//     fn from(e: E) -> Self {
//         Self::AnyErr(e.into())
//     }
// }

#[derive(Debug)]
pub struct AnyErr(Box<dyn std::error::Error + Send + Sync>);

impl<E: Into<Box<dyn std::error::Error + Send + Sync>>> From<E> for AnyErr {
    fn from(e: E) -> Self {
        Self(e.into())
    }
}
