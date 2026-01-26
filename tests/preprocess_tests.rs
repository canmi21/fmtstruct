/* tests/preprocess_tests.rs */

#![cfg(feature = "json")]

use async_trait::async_trait;
use fmtstruct::format::AnyFormat;
use fmtstruct::{DynLoader, FmtError, LoadResult, PreProcess, Source};
use serde::Deserialize;
use std::collections::HashMap;

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

#[cfg(feature = "validate")]
use validator::Validate;

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "validate", derive(Validate))]
struct TestConfig {
	#[serde(default)]
	processed: bool,
	#[serde(default)]
	ctx: String,
}
impl PreProcess for TestConfig {
	fn pre_process(&mut self) {
		self.processed = true;
	}
	fn set_context(&mut self, ctx: &str) {
		self.ctx = ctx.to_string();
	}
}

#[tokio::test]
async fn test_preprocess_hooks() {
	let mut data = HashMap::new();
	data.insert("config.json".to_string(), r#"{}"#.as_bytes().to_vec());
	let source = MockSource { data };

	let loader = DynLoader::new(Box::new(source), vec![AnyFormat::Json]);

	let result: LoadResult<TestConfig> = loader.load("config").await;
	match result {
		LoadResult::Ok(cfg) => {
			assert!(cfg.processed);
			assert_eq!(cfg.ctx, "config.json");
		}
		_ => panic!("Expected Ok, got {:?}", result),
	}
}
