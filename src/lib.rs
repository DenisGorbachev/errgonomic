//! Macros for ergonomic error handling with [thiserror](https://crates.io/crates/thiserror).
//!
//! ## Example
//!
//! ```rust
//! # #[cfg(feature = "std")]
//! # {
//! # use std::io;
//! # use std::fs::read_to_string;
//! # use std::path::{Path, PathBuf};
//! # use serde::{Deserialize, Serialize};
//! # use serde_json::from_str;
//! # use thiserror::Error;
//! # use errgonomic::handle;
//! #
//! #[derive(Serialize, Deserialize)]
//! struct Config {/* some fields */}
//!
//! // bad: doesn't return the path to config (the user won't be able to fix it)
//! fn parse_config_v1(path: PathBuf) -> io::Result<Config> {
//!     let contents = read_to_string(&path)?;
//!     let config = from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
//!     Ok(config)
//! }
//!
//! // good: returns the path to config & the underlying deserialization error (the user will be able fix it)
//! fn parse_config_v2(path: PathBuf) -> Result<Config, ParseConfigError> {
//!     use ParseConfigError::*;
//!     let contents = handle!(read_to_string(&path), ReadToStringFailed, path);
//!     let config = handle!(from_str(&contents), DeserializeFailed, path, contents);
//!     Ok(config)
//! }
//!
//! #[derive(Error, Debug)]
//! enum ParseConfigError {
//!     #[error("failed to read file to string: '{path}'")]
//!     ReadToStringFailed { path: PathBuf, source: std::io::Error },
//!     #[error("failed to parse the file contents into config: '{path}'")]
//!     DeserializeFailed { path: PathBuf, contents: String, source: serde_json::Error }
//! }
//! # }
//! ```
//!
//! Advantages:
//!
//! * `parse_config_v2` allows you to determine exactly what error has occurred
//! * `parse_config_v2` provides you with all information needed to fix the underlying issue
//! * `parse_config_v2` allows you to retry the call by reusing the `path` (avoiding unnecessary clones)
//!
//! Disadvantages:
//!
//! * `parse_config_v2` is longer
//!
//! That means `parse_config_v2` is strictly better but requires writing more code. However, with LLMs, writing more code is not an issue. Therefore, it's better to use a more verbose approach `v2`, which provides you with better errors.
//!
//! This crates provides the `handle` family of macros to simplify the error handling code.
//!
//! ## Better debugging
//!
//! To improve your debugging experience: call [`exit_result`] in `main` right before return, and it will display all information necessary to understand the root cause of the error:
//!
//! ```rust
//! # #[cfg(feature = "std")]
//! # {
//! # use errgonomic::exit_result;
//! # use thiserror::Error;
//! # use std::process::ExitCode;
//! #
//! # #[derive(Error, Debug)]
//! # enum Err {}
//! #
//! # fn run() -> Result<ExitCode, Err> { Ok(ExitCode::SUCCESS) }
//! #
//! pub fn main() -> ExitCode {
//!     exit_result(run())
//! }
//! # }
//! ```
//!
//! This will produce a nice "error trace" like below:
#![doc = "```text"]
#![doc = include_str!("./functions/writeln_error/fixtures/must_write_error.txt")]
#![doc = "```"]
//!

#![deny(clippy::arithmetic_side_effects)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

mod macros;

mod types;

pub use types::*;

mod functions;

pub use functions::*;

#[cfg(all(test, feature = "std"))]
mod drafts;
