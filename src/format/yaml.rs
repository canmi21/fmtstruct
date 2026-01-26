/* src/format/yaml.rs */

use crate::{FmtError, Format};
use serde::de::DeserializeOwned;

/// YAML format parser using `serde_yaml`.
pub struct Yaml;

impl Format for Yaml {
	fn extensions(&self) -> &'static [&'static str] {
		&["yaml", "yml"]
	}

	fn parse<T: DeserializeOwned>(&self, input: &[u8]) -> Result<T, FmtError> {
		serde_yaml::from_slice(input).map_err(|e| {
			#[cfg(feature = "alloc")]
			{
				FmtError::ParseError(alloc::format!("{}", e))
			}
			#[cfg(not(feature = "alloc"))]
			{
				_ = e;
				FmtError::ParseError
			}
		})
	}
}
