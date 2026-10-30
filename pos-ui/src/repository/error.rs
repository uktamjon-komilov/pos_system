//! Errors any repository implementation can report to the controllers.
//! A real API client will add a transport variant (timeouts, 5xx, ...).

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepositoryError {
    NotFound { entity: &'static str, id: String },
}

impl RepositoryError {
    pub fn not_found(entity: &'static str, id: &str) -> RepositoryError {
        RepositoryError::NotFound {
            entity,
            id: id.to_string(),
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound { entity, id } => {
                write!(formatter, "{entity} {id} not found")
            }
        }
    }
}

impl std::error::Error for RepositoryError {}
