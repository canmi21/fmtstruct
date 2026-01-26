/* src/format/toml.rs */

use crate::{FmtError, Format};
use serde::de::DeserializeOwned;

/// TOML format parser using `toml`.
pub struct Toml;

impl Format for Toml {
	fn extensions(&self) -> &'static [&'static str] {
		&["toml"]
	}

	fn parse<T: DeserializeOwned>(&self, input: &[u8]) -> Result<T, FmtError> {
		let s = core::str::from_utf8(input).map_err(|e| {
			#[cfg(feature = "alloc")]
			{
				FmtError::ParseError(alloc::format!("{}", e))
			}
			#[cfg(not(feature = "alloc"))]
			{
				_ = e;
				FmtError::ParseError
			}
		})?;
		toml::from_str(s).map_err(|e| {
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
