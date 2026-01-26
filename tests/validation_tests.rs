/* tests/validation_tests.rs */

use async_trait::async_trait;
use fmtstruct::format::AnyFormat;
use fmtstruct::{DynLoader, FmtError, LoadResult, PreProcess, Source};
use serde::Deserialize;
use std::collections::HashMap;
use validator::Validate;

#[derive(Default)]
struct MockSource {
	data: HashMap<String, Vec<u8>>,
}

#[async_trait]
impl Source for MockSource {
	async fn read(&self, key: &str) -> Result<Vec<u8>, FmtError> {
		self.data.get(key).cloned().ok_or(FmtError::NotFound)
	}
	async fn exists(&self, key: &str) -> bool {
		self.data.contains_key(key)
	}
}

#[derive(Debug, Deserialize, Validate, Default)]
struct TestConfig {
	#[validate(length(min = 3))]
	name: String,
	#[cfg(feature = "regex")]
	#[validate(regex(path = *RE_EMAIL))]
	email: String,
}

impl PreProcess for TestConfig {}

#[cfg(feature = "regex")]
use std::sync::LazyLock;
#[cfg(feature = "regex")]
static RE_EMAIL: LazyLock<regex::Regex> =
	LazyLock::new(|| regex::Regex::new(r"^\w+@\w+\.\w+$").unwrap());

#[tokio::test]
async fn test_validation_success() {
	let mut data = HashMap::new();
	let content = if cfg!(feature = "regex") {
		r#"{ "name": "alice", "email": "alice@example.com" }"#
	} else {
		r#"{ "name": "alice" }"#
	};
	data.insert("config.json".to_string(), content.as_bytes().to_vec());
	let source = MockSource { data };

	let loader = DynLoader::new(Box::new(source), vec![AnyFormat::Json]);

	let result: LoadResult<TestConfig> = loader.load("config").await;
	assert!(matches!(result, LoadResult::Ok(_)));
}

#[tokio::test]
async fn test_validation_failure() {
	let mut data = HashMap::new();
	let content = if cfg!(feature = "regex") {
		r#"{ "name": "al", "email": "alice@example.com" }"#
	} else {
		r#"{ "name": "al" }"#
	};
	data.insert("config.json".to_string(), content.as_bytes().to_vec());
	let source = MockSource { data };

	let loader = DynLoader::new(Box::new(source), vec![AnyFormat::Json]);

	let result: LoadResult<TestConfig> = loader.load("config").await;
	match result {
		LoadResult::Invalid(FmtError::Validation(_)) => {}
		_ => panic!("Expected Validation error, got {:?}", result),
	}
}

#[cfg(feature = "regex")]
#[tokio::test]
async fn test_regex_validation_failure() {
	let mut data = HashMap::new();
	data.insert(
		"config.json".to_string(),
		r#"{ "name": "alice", "email": "invalid-email" }"#.as_bytes().to_vec(),
	);
	let source = MockSource { data };

	let loader = DynLoader::new(Box::new(source), vec![AnyFormat::Json]);

	let result: LoadResult<TestConfig> = loader.load("config").await;
	match result {
		LoadResult::Invalid(FmtError::Validation(_)) => {}
		_ => panic!("Expected Validation error for regex, got {:?}", result),
	}
}
