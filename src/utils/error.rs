use std::{error, panic::Location};

use log::error;

#[derive(Debug)]
pub(crate) enum ErrorKind {
    String(String),
    Error(Box<dyn error::Error + Send + Sync>),
    OptNone,
    Poisoned,
}

#[derive(Debug)]
pub(crate) struct Error {
    kind: ErrorKind,
    location: Location<'static>,
}

impl<T: error::Error + Send + Sync + 'static> From<T> for Error {
    #[inline]
    #[track_caller]
    fn from(err: T) -> Self {
        Error { location: *Location::caller(), kind: ErrorKind::Error(Box::new(err)) }
    }
}

impl Error {
    #[inline]
    #[track_caller]
    pub(crate) fn new(err: &str) -> Self {
        Error { location: *Location::caller(), kind: ErrorKind::String(err.to_string()) }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn noneopt() -> Self {
        Error { location: *Location::caller(), kind: ErrorKind::OptNone }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn poisoned() -> Self {
        Error { location: *Location::caller(), kind: ErrorKind::Poisoned }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn log(&self) {
        error!("{:?} at {:?}, logged at {}", self.kind, self.location, Location::caller());
    }
}

impl From<Error> for binder::Status {
    #[track_caller]
    fn from(err: Error) -> Self {
        error!("Casting {:?} at {}, to binder at {}", err.kind, err.location, Location::caller());
        binder::Status::from(binder::StatusCode::UNKNOWN_ERROR)
    }
}
