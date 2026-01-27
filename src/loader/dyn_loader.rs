/* src/loader/dyn_loader.rs */

#[cfg(feature = "alloc")]
use crate::{FmtError, Format, LoadResult, PreProcess, Source, ValidateConfig, format::AnyFormat};
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
impl core::fmt::Debug for DynLoader {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("DynLoader")
			.field("source", &"<dyn Source>")
			.field("formats", &self.formats)
			.finish()
	}
}

#[cfg(feature = "alloc")]
pub struct DynLoaderBuilder {
	source: Option<Box<dyn Source>>,
	formats: Vec<AnyFormat>,
}

#[cfg(feature = "alloc")]
impl DynLoaderBuilder {
	pub fn new() -> Self {
		Self {
			source: None,
			formats: Vec::new(),
		}
	}

	pub fn source(mut self, source: impl Source + 'static) -> Self {
		self.source = Some(Box::new(source));
		self
	}

	pub fn format(mut self, format: AnyFormat) -> Self {
		self.formats.push(format);
		self
	}

	pub fn build(self) -> Result<DynLoader, &'static str> {
		let source = self.source.ok_or("source is required")?;
		if self.formats.is_empty() {
			return Err("at least one format is required");
		}
		Ok(DynLoader {
			source,
			formats: self.formats,
		})
	}
}

#[cfg(feature = "alloc")]
impl DynLoader {
	pub fn new(source: Box<dyn Source>, formats: Vec<AnyFormat>) -> Self {
		Self { source, formats }
	}

	pub fn builder() -> DynLoaderBuilder {
		DynLoaderBuilder::new()
	}

	/// Automatically detects and loads the configuration based on registered formats.
	pub async fn load<T>(&self, base_name: &str) -> LoadResult<T>
	where
		T: DeserializeOwned + PreProcess + ValidateConfig,
	{
		let mut found: Option<(alloc::string::String, &AnyFormat)> = None;
		let mut conflicts = Vec::new();

		for format in &self.formats {
			for ext in format.extensions() {
				let key = alloc::format!("{}.{}", base_name, ext);
				if self.source.exists(&key).await {
					if found.is_some() {
						conflicts.push(key);
					} else {
						found = Some((key, format));
					}
				}
			}
		}

		if let Some((key, format)) = found {
			self.load_explicit(&key, format, conflicts).await
		} else {
			LoadResult::NotFound
		}
	}

	/// Directly loads a specific path, selecting parser by extension.
	pub async fn load_file<T>(&self, path: &str) -> LoadResult<T>
	where
		T: DeserializeOwned + PreProcess + ValidateConfig,
	{
		let ext = if let Some(idx) = path.rfind('.') {
			&path[idx + 1..]
		} else {
			#[cfg(feature = "alloc")]
			return LoadResult::Invalid(FmtError::ParseError(alloc::string::String::from(
				"missing extension",
			)));
			#[cfg(not(feature = "alloc"))]
			return LoadResult::Invalid(FmtError::ParseError);
		};

		for format in &self.formats {
			if format.extensions().contains(&ext) {
				return self.load_explicit(path, format, Vec::new()).await;
			}
		}
		LoadResult::NotFound
	}

	/// Dry-run mode, validates without returning data.
	#[cfg(feature = "validate")]
	pub async fn validate<T>(&self, base_name: &str) -> Result<(), FmtError>
	where
		T: DeserializeOwned + PreProcess + validator::Validate,
	{
		match self.load::<T>(base_name).await {
			LoadResult::Ok { .. } => Ok(()),
			LoadResult::Invalid(e) => Err(e),
			LoadResult::NotFound => Err(FmtError::NotFound),
		}
	}

	/// Loads the configuration using a specific key and format.
	async fn load_explicit<T>(
		&self,
		key: &str,
		format: &AnyFormat,
		conflicts: Vec<alloc::string::String>,
	) -> LoadResult<T>
	where
		T: DeserializeOwned + PreProcess + ValidateConfig,
	{
		let bytes = match self.source.read(key).await {
			Ok(b) => b,
			Err(FmtError::NotFound) => return LoadResult::NotFound,
			Err(e) => return LoadResult::Invalid(e),
		};

		match format.parse::<T>(&bytes) {
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
						format: format.extensions().first().copied().unwrap_or("unknown"),
						#[cfg(feature = "std")]
						conflicts: conflicts
							.into_iter()
							.map(std::path::PathBuf::from)
							.collect(),
						#[cfg(not(feature = "std"))]
						conflicts,
					},
				}
			}
			Err(e) => LoadResult::Invalid(e),
		}
	}
}
