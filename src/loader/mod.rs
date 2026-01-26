/* src/loader/mod.rs */

mod static_loader;
pub use static_loader::StaticLoader;

#[cfg(feature = "alloc")]
mod dyn_loader;
#[cfg(feature = "alloc")]
pub use dyn_loader::DynLoader;
