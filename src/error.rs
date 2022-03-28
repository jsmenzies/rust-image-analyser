use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum ApplicationError {
    UnsupportedFileType(PathBuf, String),
    UnknownFileType(PathBuf, String),
    LibraryParseError(PathBuf, String),
    ExifParseError(PathBuf, String),
    NoFileName(PathBuf, String),
    IOError(PathBuf, String),
}