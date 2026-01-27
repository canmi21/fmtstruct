# Fmtstruct

Format-agnostic configuration loader supporting no_std, alloc, and std.

`fmtstruct` provides a flexible, compile-time or runtime-extensible interface for loading and parsing configuration files from various sources (Memory, FileSystem) into Rust structs, with optional validation and preprocessing hooks.

## Features

- **Format Agnostic**: Support for multiple formats (`JSON`, `TOML`, `YAML`, `Postcard`) via feature flags.
- **Flexible Loading**:
  - `StaticLoader`: Zero-cost abstraction for compile-time defined source/format pairs.
  - `DynLoader`: Runtime automatic format detection and dynamic dispatch.
- **Source Abstraction**:
  - `MemorySource`: Useful for testing or embedded `no_std` environments.
  - `FileSource`: Secure file system access with sandbox protection against path traversal.
- **Advanced Lifecycle**:
  - `PreProcess`: Hooks for data normalization or context injection before validation.
  - `ValidateConfig`: Optional integration with the `validator` crate for struct validation.
- **Environment Support**: designed for `no_std` (requires `alloc`), `alloc`, and `std` environments seamlessly.

## Usage Examples

Check the `examples` directory for runnable code:

- **Basic Usage**: [`examples/basic.rs`](examples/basic.rs) - Load a configuration from a file with automatic format detection.
- **Dynamic Loading**: [`tests/loader_tests.rs`](tests/loader_tests.rs) - Examples of auto-detecting formats and handling parsing errors.
- **Validation**: [`tests/validation_tests.rs`](tests/validation_tests.rs) - Integrate `validator` to enforce rules on configuration fields.
- **Preprocessing**: [`tests/preprocess_tests.rs`](tests/preprocess_tests.rs) - Inject context (like filenames) into the configuration struct during loading.
- **No-Std/Embedded**: [`src/source/memory.rs`](src/source/memory.rs) - Use `MemorySource` for environments without a file system.

## Installation

```toml
[dependencies]
fmtstruct = { version = "0.2", features = ["full"] }
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `std` | Enables standard library support (path handling, better error reporting). |
| `alloc` | Enables heap allocation support (required for `DynLoader` and most formats). |
| `fs` | Enables `FileSource` for loading configuration from the filesystem. |
| `json` | Enables JSON format support. |
| `toml` | Enables TOML format support. |
| `yaml` | Enables YAML format support. |
| `postcard` | Enables Postcard (binary) format support (no_std). |
| `validate` | Enables configuration validation via the `validator` crate. |
| `regex` | Enables regex validation support (requires `validate`). |
| `full` | Enables all features above. |

## License

Released under the MIT License © 2026 [Canmi](https://github.com/canmi21)
