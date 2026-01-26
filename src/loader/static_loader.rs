/* src/loader/static_loader.rs */

use crate::{Format, LoadResult, PreProcess, Source};
use serde::de::DeserializeOwned;

/// A zero-cost loader that combines a specific Source and Format at compile time.
pub struct StaticLoader<S, F> {
	source: S,
	format: F,
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
		T: DeserializeOwned + PreProcess,
	{
		// Source::read returns Result<Vec<u8>, ...> (requires alloc)
		let bytes = match self.source.read(key).await {
			Ok(b) => b,
			Err(crate::FmtError::NotFound) => return LoadResult::NotFound,
			Err(e) => return LoadResult::Invalid(e),
		};

		match self.format.parse::<T>(&bytes) {
			Ok(mut obj) => {
				obj.pre_process();
				LoadResult::Ok(obj)
			}
			Err(e) => LoadResult::Invalid(e),
		}
	}
}
