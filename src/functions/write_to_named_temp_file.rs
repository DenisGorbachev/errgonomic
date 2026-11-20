use crate::{handle, map_err};
use std::env::{temp_dir, var_os};
use std::fs::{File, create_dir_all};
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{NamedTempFile, PersistError};
use thiserror::Error;

/// Persists the buffer in `ERRGONOMIC_DIR`, or the system temporary directory when unset.
pub fn write_to_named_temp_file(buf: &[u8]) -> Result<(File, PathBuf), WriteToNamedTempFileError> {
    use WriteToNamedTempFileError::*;
    let dir = var_os("ERRGONOMIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(temp_dir);
    handle!(create_dir_all(&dir), CreateDirAllFailed, dir);
    let mut temp = handle!(NamedTempFile::new_in(&dir), CreateTempFileFailed, dir);
    handle!(temp.write_all(buf), WriteFailed);
    map_err!(temp.keep(), KeepFailed)
}

/// Errors returned by [`write_to_named_temp_file`].
#[derive(Error, Debug)]
pub enum WriteToNamedTempFileError {
    #[error("failed to create error report directory '{dir}'", dir = dir.display())]
    CreateDirAllFailed { source: io::Error, dir: PathBuf },
    /// Failed to create a temporary file.
    #[error("failed to create an error report in '{dir}'", dir = dir.display())]
    CreateTempFileFailed { source: io::Error, dir: PathBuf },
    /// Failed to write the buffer into the temporary file.
    #[error("failed to write to a temporary file")]
    WriteFailed { source: io::Error },
    /// Failed to persist the temporary file to its final path.
    #[error("failed to persist the temporary file")]
    KeepFailed { source: PersistError },
}
