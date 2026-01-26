/* tests/format_tests.rs */

#[cfg(feature = "json")]
use fmtstruct::format::Json;
#[cfg(feature = "postcard")]
use fmtstruct::format::Postcard;
#[cfg(feature = "toml")]
use fmtstruct::format::Toml;
#[cfg(feature = "yaml")]
use fmtstruct::format::Yaml;

#[cfg(any(
	feature = "json",
	feature = "toml",
	feature = "yaml",
	feature = "postcard"
))]
use fmtstruct::{FmtError, Format};
#[cfg(any(
	feature = "json",
	feature = "toml",
	feature = "yaml",
	feature = "postcard"
))]
use serde::Deserialize;

#[cfg(any(
	feature = "json",
	feature = "toml",
	feature = "yaml",
	feature = "postcard"
))]
#[derive(Debug, Deserialize, PartialEq)]
struct TestConfig {
	name: String,
	value: i32,
}

#[cfg(feature = "json")]
#[test]
fn test_json_format() {
	let json = Json;
	let data = r#"{ "name": "json", "value": 1 }"#.as_bytes();
	let cfg: TestConfig = json.parse(data).unwrap();
	assert_eq!(
		cfg,
		TestConfig {
			name: "json".to_string(),
			value: 1
		}
	);
}

#[cfg(feature = "toml")]
#[test]
fn test_toml_format() {
	let toml = Toml;
	let data = r###"name = "toml"
value = 2"###
		.as_bytes();
	let cfg: TestConfig = toml.parse(data).unwrap();
	assert_eq!(
		cfg,
		TestConfig {
			name: "toml".to_string(),
			value: 2
		}
	);
}

#[cfg(feature = "yaml")]
#[test]
fn test_yaml_format() {
	let yaml = Yaml;
	let data = "name: yaml\nvalue: 3".as_bytes();
	let cfg: TestConfig = yaml.parse(data).unwrap();
	assert_eq!(
		cfg,
		TestConfig {
			name: "yaml".to_string(),
			value: 3
		}
	);
}

#[cfg(feature = "postcard")]
#[test]
fn test_postcard_format() {
	use serde::Serialize;
	#[derive(Serialize, Deserialize)]
	struct PostcardTest {
		name: String,
		value: i32,
	}

	let original = PostcardTest {
		name: "postcard".to_string(),
		value: 4,
	};
	let data = postcard::to_stdvec(&original).unwrap();

	let pc = Postcard;
	let cfg: PostcardTest = pc.parse(&data).unwrap();
	assert_eq!(cfg.name, "postcard");
	assert_eq!(cfg.value, 4);
}

#[cfg(feature = "json")]
#[test]
fn test_parse_error() {
	let json = Json;
	let result: Result<TestConfig, FmtError> = json.parse(b"invalid");
	#[cfg(feature = "alloc")]
	assert!(matches!(result, Err(FmtError::ParseError(_))));
	#[cfg(not(feature = "alloc"))]
	assert!(matches!(result, Err(FmtError::ParseError)));
}
