/* src/loader/dyn_loader.rs */

#[cfg(feature = "alloc")]
use crate::{FmtError, Format, LoadResult, PreProcess, Source, format::AnyFormat};
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use serde::de::DeserializeOwned;

#[cfg(feature = "alloc")]
pub struct DynLoader {
	source: Box<dyn Source>,
	formats: Vec<AnyFormat>,
}

#[cfg(feature = "alloc")]
impl DynLoader {
	pub fn new(source: Box<dyn Source>, formats: Vec<AnyFormat>) -> Self {
		Self { source, formats }
	}

	/// Automatically detects and loads the configuration based on registered formats.
	pub async fn load<T>(&self, base_name: &str) -> LoadResult<T>
	where
		T: DeserializeOwned + PreProcess,
	{
		for format in &self.formats {
			for ext in format.extensions() {
				let key = alloc::format!("{}.{}", base_name, ext);
				if self.source.exists(&key).await {
					return self.load_explicit(&key, format).await;
				}
			}
		}
		LoadResult::NotFound
	}

	/// Loads the configuration using a specific key and format.
	async fn load_explicit<T>(&self, key: &str, format: &AnyFormat) -> LoadResult<T>
	where
		T: DeserializeOwned + PreProcess,
	{
		let bytes = match self.source.read(key).await {
			Ok(b) => b,
			Err(FmtError::NotFound) => return LoadResult::NotFound,
			Err(e) => return LoadResult::Invalid(e),
		};

		match format.parse::<T>(&bytes) {
			Ok(mut obj) => {
				obj.pre_process();
				LoadResult::Ok(obj)
			}
			Err(e) => LoadResult::Invalid(e),
		}
	}
}
