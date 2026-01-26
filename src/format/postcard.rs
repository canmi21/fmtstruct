/* src/format/postcard.rs */

use crate::{FmtError, Format};
use serde::de::DeserializeOwned;

/// Postcard format parser using `postcard`.
pub struct Postcard;

impl Format for Postcard {
	fn extensions(&self) -> &'static [&'static str] {
		&["bin", "post"]
	}

	fn parse<T: DeserializeOwned>(&self, input: &[u8]) -> Result<T, FmtError> {
		postcard::from_bytes(input).map_err(|e| {
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
