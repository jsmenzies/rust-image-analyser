use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ApplicationError;
use crate::extension::{Extension, match_extension};

#[derive(Debug, Default, Clone)]
pub struct FileDetails {
    pub path: PathBuf,
    pub title: OsString,
    pub extension: Extension,
    pub content_length: u64,
}

pub fn parse_file_details(path: &Path) -> Result<FileDetails, ApplicationError> {
    Ok(FileDetails {
        path: path.to_path_buf(),
        title: parse_title(path)?,
        extension: parse_extension(path)?,
        content_length: parse_fs_content_length(path)?,
    })
}

fn parse_title(path: &Path) -> Result<OsString, ApplicationError> {
    match path.file_name() {
        Some(value) => Ok(value.to_os_string()),
        None => {
            // Unsure if the name can actually not be present?
            Err(ApplicationError::NoFileName(PathBuf::from(path),
                                             "No file name detected".to_string()))
        }
    }
}

fn parse_extension(path: &Path) -> Result<Extension, ApplicationError> {
    let io_ext = path.extension();

    if io_ext.is_none() {
        return Err(ApplicationError::UnknownFileType(PathBuf::from(path),
                                                     "Unknown file extension".to_string()));
    }

    let extension = match_extension(io_ext.unwrap());
    if extension != Extension::UNKNOWN {
        Ok(extension)
    } else {
        Err(ApplicationError::UnsupportedFileType(PathBuf::from(path),
                                                  "Unsupported file extension".to_string()))
    }
}

fn parse_fs_content_length(path: &Path) -> Result<u64, ApplicationError> {
    fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| ApplicationError::IOError(path.to_path_buf(), e.to_string()))
}
