/* tests/loader_tests.rs */

use async_trait::async_trait;
use fmtstruct::format::AnyFormat;
use fmtstruct::{DynLoader, FmtError, LoadResult, PreProcess, Source};
use serde::Deserialize;
use std::collections::HashMap;

// --- Mocks ---

#[derive(Default)]
struct MockSource {
	data: HashMap<String, Vec<u8>>,
}

impl MockSource {
	fn new() -> Self {
		Self::default()
	}

	fn insert(&mut self, key: &str, content: &str) {
		self
			.data
			.insert(key.to_string(), content.as_bytes().to_vec());
	}
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

#[derive(Debug, Deserialize, Default)]
struct TestConfig {
	name: String,
	value: i32,
	#[serde(skip)]
	context: String,
}

impl PreProcess for TestConfig {
	fn set_context(&mut self, ctx: &str) {
		self.context = ctx.to_string();
	}
}

#[cfg(feature = "validate")]
impl validator::Validate for TestConfig {
	fn validate(&self) -> Result<(), validator::ValidationErrors> {
		if self.value < 0 {
			let mut errors = validator::ValidationErrors::new();
			errors.add("value", validator::ValidationError::new("invalid_value"));
			return Err(errors);
		}
		Ok(())
	}
}
// --- Tests ---

#[tokio::test]
async fn test_dyn_loader_auto_detect() {
	let mut source = MockSource::new();
	source.insert("config.json", r#"{ "name": "test", "value": 42 }"#);

	let loader = DynLoader::new(Box::new(source), vec![AnyFormat::Json]);

	let result: LoadResult<TestConfig> = loader.load("config").await;

	match result {
		LoadResult::Ok(cfg) => {
			assert_eq!(cfg.name, "test");
			assert_eq!(cfg.value, 42);
			assert_eq!(cfg.context, "config.json");
		}
		_ => panic!("Expected Ok result, got {:?}", result),
	}
}

#[tokio::test]
async fn test_dyn_loader_load_file() {
	let mut source = MockSource::new();
	source.insert("specific.json", r#"{ "name": "file", "value": 10 }"#);

	let loader = DynLoader::new(Box::new(source), vec![AnyFormat::Json]);

	let result: LoadResult<TestConfig> = loader.load_file("specific.json").await;

	match result {
		LoadResult::Ok(cfg) => {
			assert_eq!(cfg.name, "file");
			assert_eq!(cfg.value, 10);
			assert_eq!(cfg.context, "specific.json");
		}
		_ => panic!("Expected Ok result, got {:?}", result),
	}
}

#[tokio::test]
async fn test_not_found() {
	let source = MockSource::new();
	let loader = DynLoader::new(Box::new(source), vec![AnyFormat::Json]);

	let result: LoadResult<TestConfig> = loader.load("missing").await;
	match result {
		LoadResult::NotFound => {}
		_ => panic!("Expected NotFound, got {:?}", result),
	}
}

#[tokio::test]
async fn test_parse_error() {
	let mut source = MockSource::new();
	source.insert("config.json", "invalid json");

	let loader = DynLoader::new(Box::new(source), vec![AnyFormat::Json]);

	let result: LoadResult<TestConfig> = loader.load("config").await;
	match result {
		#[cfg(feature = "alloc")]
		LoadResult::Invalid(FmtError::ParseError(_)) => {}
		#[cfg(not(feature = "alloc"))]
		LoadResult::Invalid(FmtError::ParseError) => {}
		_ => panic!("Expected Invalid(ParseError), got {:?}", result),
	}
}
