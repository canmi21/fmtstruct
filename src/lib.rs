/* src/lib.rs */

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

pub mod error;
pub mod format;
pub mod loader;
pub mod source;

// Re-export core types
pub use error::FmtError;
pub use loader::StaticLoader;

#[cfg(feature = "alloc")]
pub use loader::DynLoader;

#[cfg(feature = "alloc")]
pub use source::MemorySource;

#[cfg(feature = "fs")]
pub use source::FileSource;

#[cfg(feature = "alloc")]
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Result of a loading operation.
#[derive(Debug)]
pub enum LoadResult<T> {
	/// Successfully loaded and parsed.
	Ok(T),
	/// Resource not found at the given key.
	NotFound,
	/// Resource exists but is invalid.
	Invalid(FmtError),
}

/// A hook to process data after parsing but before validation.
pub trait PreProcess {
	/// Perform data normalization or context injection.
	fn pre_process(&mut self) {}
	/// Set context information (e.g., file path or key).
	fn set_context(&mut self, _ctx: &str) {}
}

/// Internal trait for optional validation.
#[cfg(feature = "validate")]
pub trait ValidateConfig: validator::Validate {
	fn validate_config(&self) -> Result<(), FmtError> {
		self.validate().map_err(|e| {
			#[cfg(feature = "std")]
			{
				FmtError::Validation(e)
			}
			#[cfg(not(feature = "std"))]
			{
				_ = e;
				FmtError::Validation
			}
		})
	}
}

#[cfg(feature = "validate")]
impl<T: validator::Validate> ValidateConfig for T {}

#[cfg(not(feature = "validate"))]
pub trait ValidateConfig {
	fn validate_config(&self) -> Result<(), FmtError> {
		Ok(())
	}
}

#[cfg(not(feature = "validate"))]
impl<T> ValidateConfig for T {}

/// Abstract format parser that converts bytes into a structured object.
///
/// Note: This trait is not object-safe due to the generic `parse` method.
/// Use `format::AnyFormat` for dynamic dispatch.
pub trait Format: Send + Sync {
	/// List of supported extensions or identifiers.
	fn extensions(&self) -> &'static [&'static str];

	/// Parse the raw bytes into the target type.
	fn parse<T: DeserializeOwned>(&self, input: &[u8]) -> Result<T, FmtError>;
}

/// Abstract data source that retrieves raw bytes by key.
///
/// In `alloc` mode, this uses `async_trait` for object safety (allowing `dyn Source`).
/// In `no_alloc` mode, this uses native `async fn` (static dispatch only).
#[cfg_attr(feature = "alloc", async_trait)]
pub trait Source: Send + Sync {
	/// Read raw data as a vector of bytes.
	#[cfg(feature = "alloc")]
	async fn read(&self, key: &str) -> Result<alloc::vec::Vec<u8>, FmtError>;

	/// Check if the resource exists at the given key.
	async fn exists(&self, key: &str) -> bool;
}
