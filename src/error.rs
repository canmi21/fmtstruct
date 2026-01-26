/* src/error.rs */

#[cfg(not(feature = "std"))]
use core::fmt;

/// Core error type for the fmtstruct crate.
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum FmtError {
	/// Parsing error from format implementation.
	#[cfg(feature = "alloc")]
	#[cfg_attr(feature = "std", error("parse error: {0}"))]
	ParseError(alloc::string::String),

	/// Parsing error from format implementation.
	#[cfg(not(feature = "alloc"))]
	#[cfg_attr(feature = "std", error("parse error"))]
	ParseError,

	/// Resource not found.
	#[cfg_attr(feature = "std", error("not found"))]
	NotFound,

	/// Generic static error message.
	#[cfg_attr(feature = "std", error("custom error: {0}"))]
	Custom(&'static str),

	/// IO error from source, only available in std environment.
	#[cfg(feature = "std")]
	#[error("io error: {0}")]
	Io(#[from] std::io::Error),

	/// Sandbox violation in file system source.
	#[cfg(feature = "fs")]
	#[error("sandbox violation")]
	SandboxViolation,

	/// Validation error from validator crate.
	#[cfg(feature = "validate")]
	#[cfg_attr(feature = "std", error("validation failed: {0}"))]
	Validation(#[cfg(feature = "std")] validator::ValidationErrors),
}

#[cfg(not(feature = "std"))]
impl fmt::Display for FmtError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			#[cfg(feature = "alloc")]
			Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
			#[cfg(not(feature = "alloc"))]
			Self::ParseError => write!(f, "Parse error"),
			Self::NotFound => write!(f, "Not found"),
			Self::Custom(s) => write!(f, "Custom error: {}", s),
			#[cfg(feature = "fs")]
			Self::SandboxViolation => write!(f, "Sandbox violation"),
			#[cfg(feature = "validate")]
			Self::Validation(_) => write!(f, "Validation error"),
		}
	}
}
