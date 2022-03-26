use std::{fs, io};
use std::default::Default;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};
use std::path::{Path, PathBuf};

use imagesize::blob_size;
use md5::Digest;

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

#[derive(Debug, Default)]
pub struct Location {
    root: PathBuf,
    paths: Vec<PathBuf>,
    pub metadata: Vec<Metadata>,
}

#[derive(Default, Debug)]
pub struct Metadata {
    path: PathBuf,
    title: OsString,
    extension: Extension,
    content_length: u64,
    pub errors: Vec<Error>,
    md5: String,
    width: u32,
    height: u32,
    // title_date_time: String,
    // file_modified_date: String,
    // exif_id0_date: String,
    // exif_sub_id0_date_original: String,
    // exif_sub_id0_date_digital: String,
    // quicktime_meta_create_date: String,
    // gps_date: String,
    // iptc_date_create: String,
    // iptc_digital_date_create: String,
    // png_create_date: String,
}

pub fn add_location(root_string: String) -> Result<Location, Error> {
    let path = Path::new(&root_string);
    let root = PathBuf::from(path);

    match verify_dir(path) {
        Ok(paths) => {
            let location = Location {
                root,
                paths,
                metadata: Vec::new(),
            };
            Ok(location)
        }
        Err(e) => Err(e),
    }
}

pub fn shallow_pass_location(mut location: Location) -> Location {
    for path in location.paths.iter() {
        let mut metadata = Metadata {
            path: path.to_path_buf(),
            ..Metadata::default()
        };

        match parse_fs_content_length(&metadata.path) {
            Ok(content_length) => {
                metadata.content_length = content_length;
            }
            Err(e) => {
                metadata.errors.push(e);
            }
        }

        match parse_title(&metadata.path) {
            Ok(title) => {
                metadata.title = title;
            }
            Err(e) => {
                metadata.errors.push(e);
            }
        }

        match parse_extension(&metadata.path) {
            Ok(extension) => {
                metadata.extension = extension;
            }
            Err(e) => {
                metadata.errors.push(e);
            }
        }

        if !metadata.errors.is_empty() {
            println!("{:?}", metadata.errors);
        }

        location.metadata.push(metadata);
    }


    location
}

pub fn deep_pass_location(mut location: Location) -> Location {
    for metadata in location.metadata.iter_mut() {
        decode_file_parse(metadata);
    }
    location
}

fn decode_file_parse(metadata: &mut Metadata) {
    let file = File::open(&metadata.path).unwrap();
    let mut buf_reader = BufReader::new(file);
    //
    let mut buffer = Vec::<u8>::new();
    let mut buffer = [0; 15_000_0];
    //
    // buf_reader.read_to_end(&mut buffer);
    buf_reader.read(&mut buffer);
    // let raw_bytes = fs::read(&metadata.path).unwrap();
    // let ((width, height), error) = parse_dimensions(&raw_bytes);

    // metadata.md5 = generate_md5(&raw_bytes);
    metadata.md5 = generate_md5(&buffer);
    let ((width, height), error) = parse_dimensions(&buffer);

    metadata.width = width;
    metadata.height = height;

    if let Some(e) = error {
        metadata.errors.push(Error::new(
            ErrorKind::Other,
            format!("{}", e),
        ))
    }
}

fn parse_dimensions(bytes: &[u8]) -> ((u32, u32), Option<imagesize::ImageError>) {
    let result = blob_size(bytes);

    match result {
        Ok(size) => ((size.width as u32, size.height as u32), None),
        Err(e) => ((0, 0), Some(e)),
    }
}

fn generate_md5(bytes: &[u8]) -> String {
    let output = md5::Md5::digest(bytes);
    format!("{:x}", output)
}

fn parse_fs_content_length(path: &Path) -> Result<u64, Error> {
    fs::metadata(path).map(|m| m.len())
}

fn parse_title(path: &Path) -> Result<OsString, Error> {
    let name = path.file_name();
    match name {
        Some(value) => {
            Ok(value.to_os_string())
        }
        None => {
            // Unsure if the name can actually not be present?
            Err(Error::new(ErrorKind::Other, "No file name"))
        }
    }
}

fn parse_extension(path: &Path) -> Result<Extension, Error> {
    match path.extension() {
        Some(str) => {
            let extension = match_extension(str);
            if extension != Extension::UNKNOWN {
                Ok(extension)
            } else {
                Err(Error::new(ErrorKind::Other, "Unknown file extension"))
            }
        }
        None => {
            Err(Error::new(ErrorKind::Other, "No file extension"))
        }
    }
}

fn match_extension(string: &OsStr) -> Extension {
    match string.to_ascii_lowercase().to_str().unwrap() {
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
