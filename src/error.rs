use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, AxionError>;

#[derive(Debug, Error)]
pub enum AxionError {
    #[error("Project '{0}' already exists in this directory")]
    ProjectExists(String),

    #[error("IO error: {0}")]
    Io(#[source] io::Error),

    #[error("Kind '{0}' already exists in this project")]
    KindExists(String),

    #[error("Unknown kind '{0}'. Try: route, handler, model, middleware")]
    UnknownKind(String),

    #[error("Template error: {0}")]
    Template(String),
}

impl AxionError {
    pub fn not_found(&self) -> bool {
        matches_io_kind(self, io::ErrorKind::NotFound)
    }

    pub fn permission_denid(&self) -> bool {
        matches_io_kind(self, io::ErrorKind::PermissionDenied)
    }

    pub fn already_exists(&self) -> bool {
        matches_io_kind(self, io::ErrorKind::AlreadyExists)
    }
}

fn matches_io_kind(err: &AxionError, kind: io::ErrorKind) -> bool {
    if let AxionError::Io(ref io_error) = *err {
        return io_error.kind() == kind;
    }
    false
}
