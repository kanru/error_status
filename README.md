# error_status

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/error_status.svg)](https://crates.io/crates/error_status)


A lightweight error handling library for Rust that combines HTTP-style error statuses with contextual information.

## Features

- **HTTP-inspired Error Status**: Wrap errors with semantic status codes similar to HTTP status codes
- **Context-Rich Errors**: Add meaningful context strings to your errors for better debugging
- **Result Extension Trait**: Convenient methods to construct error statuses from `Result` type

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
error_status = "0.1.0"
```

## Quick Start

```rust
use std::io::{self, ErrorKind};

use anyhow::Result;
use error_status::ResultExt;

fn find_file() -> Result<(), io::Error> {
    Err(ErrorKind::NotFound.into())
}

fn main() -> Result<()> {
    find_file()
        .not_found("Failed to read file")
        .internal_error("Config file is not available")?;
    Ok(())
}
```

## Usage

The library provides a `ResultExt` trait that extends Result with methods
corresponding to common error scenarios:

* `not_found()`: For missing resource errors
* `internal_error()`: For internal system errors
* `bad_request()`: For validation failures
* And more...

Each method accepts a context string that provides additional information about
the error. There's also a corresponding `_lazy()` version for context builder.

## Error Handling Best Practices

* Use semantic status codes to categorize errors appropriately
* Provide meaningful context messages for better error tracking
* Implement comprehensive logging throughout your codebase
* Use the error handling chain to provide multiple layers of context

## License

`MIT OR Apache-2.0`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.