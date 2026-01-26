/* src/lib.rs */

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod format;
pub mod loader;
pub mod source;

pub use error::FmtError;

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
}

/// Abstract format parser that converts bytes into a structured object.
pub trait Format: Send + Sync {
	/// List of supported extensions or identifiers.
	fn extensions(&self) -> &'static [&'static str];

	/// Parse the raw bytes into the target type.
	fn parse<T: DeserializeOwned>(&self, input: &[u8]) -> Result<T, FmtError>;
}

/// Abstract data source that retrieves raw bytes by key.
#[allow(async_fn_in_trait)]
pub trait Source: Send + Sync {
	/// Read raw data as a vector of bytes.
	#[cfg(feature = "alloc")]
	async fn read(&self, key: &str) -> Result<alloc::vec::Vec<u8>, FmtError>;

	/// Check if the resource exists at the given key.
	async fn exists(&self, key: &str) -> bool;
}
