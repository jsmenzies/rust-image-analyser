use std::{fs, io};
use std::ffi::{OsString};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub fn add_location(root_string: String) -> Location {
    let path = Path::new(&root_string);
    let root = PathBuf::from(path);

    let result = verify_dir(path);

    match result {
        Ok(paths) => {
            Location {
                root,
                paths,
                metadata: Vec::new(),
                add_error: Vec::new(),
            }
        }
        Err(e) => {
            Location {
                root,
                paths: Vec::new(),
                metadata: Vec::new(),
                add_error: vec![e],
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Location {
    root: PathBuf,
    paths: Vec<PathBuf>,
    pub metadata: Vec<Metadata>,
    add_error: Vec<Error>,
}

#[derive(Default, Debug)]
pub struct Metadata {
    id: String,
    path: PathBuf,
    title: OsString,
    extension: Extension,
    content_length: u64,
    pub errors: Vec<Error>,
}

pub fn shallow_pass_location(mut location: Location) -> Location {
    for path in location.paths.iter() {
        let mut metadata = Metadata {
            path: path.to_path_buf(),
            ..Metadata::default()
        };

        metadata = parse_fs_metadata(metadata);
        metadata = parse_title(metadata);
        metadata = parse_extension(metadata);

        location.metadata.push(metadata);
    }

    location
}

fn parse_fs_metadata(mut metadata: Metadata) -> Metadata {
    let result = fs::metadata(&metadata.path);
    match result {
        Ok(fs_metadata) => {
            metadata.content_length = fs_metadata.len();
        }
        Err(e) => {
            metadata.errors.push(e);
        }
    }

    metadata
}

fn parse_title(mut metadata: Metadata) -> Metadata {
    let name = &metadata.path.file_name();
    match name {
        Some(value) => {
            metadata.title = value.to_os_string();
        }
        None => {
            // Unsure if the name can actually not be present?
            metadata.errors.push(Error::new(ErrorKind::Other, "No file name"));
        }
    }

    metadata
}

fn parse_extension(mut metadata: Metadata) -> Metadata {
    let extension = metadata.path.extension();

    metadata.extension = extension
        .map_or_else(|| Extension::UNKNOWN, |ext| {
            match ext.to_ascii_lowercase().to_str().unwrap() {
                "jpg" => Extension::JPG,
                "png" => Extension::PNG,
                "gif" => Extension::GIF,
                "bmp" => Extension::BMP,
                "webp" => Extension::WEBP,
                "tiff" => Extension::TIFF,
                "tif" => Extension::TIFF,
                "jpeg" => Extension::JPG,
                "mp4" => Extension::MP4,
                "mov" => Extension::MOV,
                _ => Extension::UNKNOWN,
            }
        });

    if metadata.extension == Extension::UNKNOWN {
        metadata.errors.push(Error::new(
            ErrorKind::Other,
            "File extension not recognised",
        ));
    }

    metadata
}

pub fn verify_dir(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    println!("Parsing directory: {}", dir.display());

    Ok(fs::read_dir(dir)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>())
}

impl Default for Extension {
    fn default() -> Self { Extension::UNKNOWN }
}

#[derive(Debug, PartialEq)]
enum Extension {
    JPG,
    MP4,
    PNG,
    GIF,
    BMP,
    TIFF,
    ICO,
    PSD,
    WEBP,
    MOV,
    UNKNOWN,
}

#[derive(Debug)]
enum FileParseError {
    FileIsEmpty,
    FileInaccessible,
    NoMD5,
}
