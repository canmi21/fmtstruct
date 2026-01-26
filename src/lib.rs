/* src/lib.rs */

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod format;
pub mod source;
pub mod loader;

pub use error::{FmtError, LoadResult};
pub use format::Format;
pub use source::Source;
pub use loader::Loader;
