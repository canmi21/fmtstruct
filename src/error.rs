/* src/error.rs */

use core::fmt;

/// Result type for loading operations.
#[derive(Debug)]
pub enum LoadResult<T> {
    /// Successfully loaded and parsed.
    Ok(T),
    /// Resource not found.
    NotFound,
    /// Resource exists but is invalid (parse error, validation error, etc.).
    Invalid(FmtError),
}

/// Core error type for the crate.
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum FmtError {
    /// Error from the underlying data source (e.g., File IO).
    #[cfg(feature = "std")]
    #[error("source error: {0}")]
    Source(#[from] std::io::Error),

    /// Error during format parsing.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "std", error("format error: {0}"))]
    Format(alloc::string::String),
    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(feature = "std", error("format error"))]
    Format,

    /// Error during data validation.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "std", error("validation error: {0}"))]
    Validation(validator::ValidationErrors),
    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(feature = "std", error("validation error"))]
    Validation,

    /// Path traversal or sandbox violation.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "std", error("sandbox violation: {path} escapes {root}"))]
    Sandbox { 
        path: alloc::string::String, 
        root: alloc::string::String 
    },
    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(feature = "std", error("sandbox violation"))]
    Sandbox,

    /// Resource not found.
    #[cfg_attr(feature = "std", error("not found"))]
    NotFound,

    /// A generic error message.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "std", error("{0}"))]
    Message(alloc::string::String),
    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(feature = "std", error("generic error"))]
    Message(&'static str),
}

#[cfg(not(feature = "std"))]
impl fmt::Display for FmtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "alloc")]
            Self::Format(s) => write!(f, "Format error: {}", s),
            #[cfg(not(feature = "alloc"))]
            Self::Format => write!(f, "Format error"),

            #[cfg(feature = "alloc")]
            Self::Validation(_) => write!(f, "Validation error"),
            #[cfg(not(feature = "alloc"))]
            Self::Validation => write!(f, "Validation error"),

            #[cfg(feature = "alloc")]
            Self::Sandbox { .. } => write!(f, "Sandbox violation"),
            #[cfg(not(feature = "alloc"))]
            Self::Sandbox => write!(f, "Sandbox violation"),

            Self::NotFound => write!(f, "Not found"),

            #[cfg(feature = "alloc")]
            Self::Message(s) => write!(f, "{}", s),
            #[cfg(not(feature = "alloc"))]
            Self::Message(s) => write!(f, "{}", s),
        }
    }
}
