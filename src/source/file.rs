/* src/source/file.rs */

use crate::{FmtError, Source};
#[cfg(feature = "alloc")]
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

/// A file system source backed by tokio::fs.
pub struct FileSource {
	root: PathBuf,
}

impl FileSource {
	/// Create a new FileSource rooted at the given path.
	pub fn new(root: impl Into<PathBuf>) -> Self {
		Self { root: root.into() }
	}

	/// Resolves the path safely, ensuring it is within the root directory.
	async fn resolve_secure(&self, key: &str) -> Result<PathBuf, FmtError> {
		let path = self.root.join(key);

		// Basic path traversal check before canonicalization
		// (canonicalize fails if file doesn't exist)
		for component in std::path::Path::new(key).components() {
			if matches!(component, std::path::Component::ParentDir) {
				return Err(FmtError::SandboxViolation);
			}
		}

		// Resolve root to absolute path
		let canonical_root = fs::canonicalize(&self.root).await.map_err(FmtError::Io)?;

		// Resolve target path
		match fs::canonicalize(&path).await {
			Ok(canonical_path) => {
				// Ensure the resolved path starts with the resolved root
				if canonical_path.starts_with(&canonical_root) {
					Ok(canonical_path)
				} else {
					Err(FmtError::SandboxViolation)
				}
			}
			Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(FmtError::NotFound),
			Err(e) => Err(FmtError::Io(e)),
		}
	}
}

#[cfg_attr(feature = "alloc", async_trait)]
impl Source for FileSource {
	async fn read(&self, key: &str) -> Result<Vec<u8>, FmtError> {
		let path = self.resolve_secure(key).await?;
		fs::read(path).await.map_err(FmtError::Io)
	}

	async fn exists(&self, key: &str) -> bool {
		self.resolve_secure(key).await.is_ok()
	}
}
