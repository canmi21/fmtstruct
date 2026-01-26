/* tests/source_tests.rs */

#[cfg(feature = "fs")]
use fmtstruct::FileSource;
use fmtstruct::MemorySource;
use fmtstruct::{FmtError, Source};

#[tokio::test]
async fn test_memory_source_basic() {
	let mut source = MemorySource::new();
	source.insert("key", vec![1, 2, 3]);

	assert!(source.exists("key").await);
	assert!(!source.exists("missing").await);

	let data = source.read("key").await.unwrap();
	assert_eq!(data, vec![1, 2, 3]);

	let err = source.read("missing").await.unwrap_err();
	match err {
		FmtError::NotFound => {}
		_ => panic!("Expected NotFound, got {:?}", err),
	}
}

#[cfg(feature = "fs")]
#[tokio::test]
async fn test_file_source_basic() {
	use std::io::Write;
	let tmp_dir = tempfile::tempdir().unwrap();
	let file_path = tmp_dir.path().join("test.txt");
	{
		let mut file = std::fs::File::create(&file_path).unwrap();
		file.write_all(b"hello world").unwrap();
	}

	let source = FileSource::new(tmp_dir.path());

	assert!(source.exists("test.txt").await);
	assert!(!source.exists("missing.txt").await);

	let data = source.read("test.txt").await.unwrap();
	assert_eq!(data, b"hello world");
}

#[cfg(feature = "fs")]
#[tokio::test]
async fn test_file_source_sandbox() {
	let tmp_dir = tempfile::tempdir().unwrap();
	let source = FileSource::new(tmp_dir.path());

	// Try to access parent directory
	let result = source.read("../secret.txt").await;
	match result {
		Err(FmtError::SandboxViolation) => {}
		_ => panic!("Expected SandboxViolation, got {:?}", result),
	}
}
