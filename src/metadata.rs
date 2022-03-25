use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Default, Debug)]
pub struct Metadata {
    id: String,
    path: PathBuf,
    title: OsString,
    extension: Extension,
    content_length: u64,
}

impl Metadata {
    pub fn new(path: PathBuf) -> Result<Self, Vec<io::Error>> {
        let mut errors = Vec::new();

        let length = match parse_fs_metadata(&path) {
            Ok(len) => Some(len),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        let title = match parse_title(&path) {
            Ok(title) => Some(title),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        let extension = match parse_extension(&path) {
            Ok(ext) => Some(ext),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        if let (Some(content_length), Some(title), Some(extension)) = (length, title, extension) {
            Ok(Self {
                path,
                title,
                extension,
                content_length,
                ..Self::default()
            })
        } else {
            Err(errors)
        }
    }
}

fn parse_fs_metadata(path: &Path) -> Result<u64, std::io::Error> {
    std::fs::metadata(path).map(|meta| meta.len())
}

fn parse_title(path: &Path) -> Result<OsString, std::io::Error> {
    path.file_name()
        .map(std::ffi::OsStr::to_os_string)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No file name"))
}

fn parse_extension(path: &Path) -> Result<Extension, std::io::Error> {
    Ok(match path.extension().and_then(std::ffi::OsStr::to_str) {
        Some("jpg" | "jpeg") => Extension::Jpg,
        Some("tif" | "tiff") => Extension::Tiff,
        Some("bmp") => Extension::Bmp,
        Some("gif") => Extension::Gif,
        Some("mov") => Extension::Mov,
        Some("mp4") => Extension::Mp4,
        Some("png") => Extension::Png,
        Some("webp") => Extension::Webp,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File extension not recognised",
            ))
        }
    })
}

#[derive(Debug, PartialEq)]
enum Extension {
    Jpg,
    Mp4,
    Png,
    Gif,
    Bmp,
    Tiff,
    Ico,
    Psd,
    Webp,
    Mov,
    Unknown,
}

impl Default for Extension {
    fn default() -> Self {
        Extension::Unknown
    }
}
