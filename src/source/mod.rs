/* src/source/mod.rs */

#[cfg(feature = "alloc")]
mod memory;
#[cfg(feature = "alloc")]
pub use memory::MemorySource;

#[cfg(feature = "fs")]
mod file;
#[cfg(feature = "fs")]
pub use file::FileSource;
