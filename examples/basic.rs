/* examples/basic.rs */

use fmtstruct::{DynLoader, FileSource, LoadResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
	name: String,
	version: u32,
}

impl fmtstruct::PreProcess for Config {}

#[tokio::main]
async fn main() {
	// Requires creating a file first.
	tokio::fs::write("config.json", r#"{"name": "demo", "version": 1}"#)
		.await
		.unwrap();

	let source = FileSource::new(".");
	let formats = vec![fmtstruct::format::AnyFormat::Json];
	let loader = DynLoader::new(Box::new(source), formats);

	match loader.load::<Config>("config").await {
		LoadResult::Ok(cfg) => println!("Loaded: {:?}", cfg),
		LoadResult::NotFound => println!("Not found"),
		LoadResult::Invalid(e) => println!("Error: {}", e),
	}

	tokio::fs::remove_file("config.json").await.unwrap();
}
