/* src/loader/static_loader.rs */

use crate::{Format, Source};
#[cfg(feature = "alloc")]
use crate::{LoadResult, PreProcess, ValidateConfig};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use serde::de::DeserializeOwned;

/// A zero-cost loader that combines a specific Source and Format at compile time.
pub struct StaticLoader<S, F> {
	pub source: S,
	pub format: F,
}

impl<S, F> StaticLoader<S, F>
where
	S: Source,
	F: Format,
{
	/// Creates a new StaticLoader.
	pub const fn new(source: S, format: F) -> Self {
		Self { source, format }
	}

	/// Loads and parses the configuration.
	#[cfg(feature = "alloc")]
	pub async fn load<T>(&self, key: &str) -> LoadResult<T>
	where
		T: DeserializeOwned + PreProcess + ValidateConfig,
	{
		// Source::read returns Result<Vec<u8>, ...> (requires alloc)
		let bytes: Vec<u8> = match self.source.read(key).await {
			Ok(b) => b,
			Err(crate::FmtError::NotFound) => return LoadResult::NotFound,
			Err(e) => return LoadResult::Invalid(e),
		};

		match self.format.parse::<T>(&bytes) {
			Ok(mut obj) => {
				obj.pre_process();
				obj.set_context(key);

				if let Err(e) = obj.validate_config() {
					return LoadResult::Invalid(e);
				}

				LoadResult::Ok {
					value: obj,
					info: crate::LoadInfo {
						#[cfg(feature = "std")]
						path: std::path::PathBuf::from(key),
						#[cfg(not(feature = "std"))]
						key: alloc::string::String::from(key),
						format: self
							.format
							.extensions()
							.first()
							.copied()
							.unwrap_or("unknown"),
						conflicts: Vec::new(),
					},
				}
			}
			Err(e) => LoadResult::Invalid(e),
		}
	}
}
