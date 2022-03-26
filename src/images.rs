use std::{fs, fs::File, io};
use std::collections::HashMap;
use std::default::Default;
use std::ffi::{OsStr, OsString};
use std::fmt::{Debug};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use imagesize::{blob_size, ImageType};
use md5::Digest;

#[derive(Clone, Debug, PartialEq)]
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
    HEIF,
    JXL,
    UNKNOWN,
}

#[derive(Debug, Default)]
pub struct Location<> {
    root: PathBuf,
    paths: Vec<PathBuf>,
    pub metadata: Vec<Metadata>,
    pub lookup: HashMap<String, Vec<Metadata>>,
}

#[derive(Clone, Debug)]
pub enum FileParseError {
    FileDoesNotExist(PathBuf, String),
    ParseError(PathBuf, String),
    IOParseError(PathBuf, String),
    LibraryParseError { message: String },
    NoFileName(PathBuf, String),
    Unreadable(PathBuf, String),
    IOError(PathBuf, String),
}

#[derive(Clone, Default, Debug)]
pub struct Metadata {
    path: PathBuf,
    title: OsString,
    advertised_extension: Extension,
    content_length: u64,
    pub errors: Vec<FileParseError>,
    md5: String,
    parsed_extension: Extension,
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

pub fn add_location(root_string: String) -> Result<Location, FileParseError> {
    let path = Path::new(&root_string);
    let root = PathBuf::from(path);

    match verify_dir(path) {
        Ok(paths) => {
            let location = Location {
                root,
                paths,
                metadata: Vec::new(),
                lookup: HashMap::new(),
            };
            Ok(location)
        }
        Err(e) => Err(FileParseError::FileDoesNotExist(root, e.to_string())),
    }
}

pub fn shallow_pass_location(mut location: Location) -> Location {
    for path in location.paths.iter() {
        let metadata = shallow_parse(path);
        location.metadata.push(metadata);
    }
    location
}

fn shallow_parse(path: &Path) -> Metadata {
    let mut metadata = Metadata {
        path: path.to_path_buf(),
        ..Metadata::default()
    };

    match parse_fs_content_length(&metadata.path) {
        Ok(content_length) => metadata.content_length = content_length,
        Err(e) => {
            println!("{:?}", e);
            metadata.errors.push(e)
        }
    }

    match parse_title(&metadata.path) {
        Ok(title) => metadata.title = title,
        Err(e) => {
            println!("{:?}", e);
            metadata.errors.push(e)
        }
    }

    match parse_extension(&metadata.path) {
        Ok(extension) => metadata.advertised_extension = extension,
        Err(e) => {
            println!("{:?}", e);
            metadata.errors.push(e)
        }
    }
    metadata
}

pub fn deep_pass_location(location: &mut Location) -> &mut Location {
    let mut lookup = HashMap::<String, Vec<Metadata>>::new();

    for metadata in location.metadata.iter_mut() {
        let path = &metadata.path;
        let bytes = parse_first_n_bytes(path);

        match parse_dimension_from_bytes(&bytes) {
            Ok((width, height)) => {
                metadata.width = width;
                metadata.height = height;
            }
            Err(e) => {
                let error = convert_library_error_to_file_parse_error(e, path);
                metadata.errors.push(error);
            }
        }

        match parse_file_type_from_bytes(&bytes) {
            Ok(file_type) => metadata.parsed_extension = file_type,
            Err(e) => {
                let error = convert_library_error_to_file_parse_error(e, path);
                metadata.errors.push(error);
            }
        }

        metadata.md5 = generate_md5(&bytes);
        lookup.entry(metadata.md5.to_string())
            .or_insert(Vec::new())
            .push(metadata.clone());
    }
    location.lookup = lookup;

    location
}

// fn parse_first_n_bytes(path: &Path) -> Vec<u8> {
fn parse_first_n_bytes(path: &Path) -> [u8; 20_000] {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut header = [0; 20_000];
    // let mut all_bytes = Vec::new();

    reader.read_exact(&mut header);
    // reader.read_to_end(&mut all_bytes);
    header
    // all_bytes
}

fn parse_file_type_from_bytes(bytes: &[u8]) -> Result<Extension, FileParseError> {
    match imagesize::image_type(bytes) {
        Ok(format) => {
            match format {
                ImageType::Jpeg => Ok(Extension::JPG),
                ImageType::Png => Ok(Extension::PNG),
                ImageType::Gif => Ok(Extension::GIF),
                ImageType::Bmp => Ok(Extension::BMP),
                ImageType::Webp => Ok(Extension::WEBP),
                ImageType::Tiff => Ok(Extension::TIFF),
                ImageType::Heif => Ok(Extension::HEIF),
                ImageType::Jxl => Ok(Extension::JXL),
                ImageType::Psd => Ok(Extension::PSD),
            }
        }
        Err(e) => {
            Err(FileParseError::LibraryParseError {
                message: e.to_string(),
            })
        }
    }
}

fn parse_dimension_from_bytes(bytes: &[u8]) -> Result<(u32, u32), FileParseError> {
    let result = blob_size(bytes);

    match result {
        Ok(size) => Ok((size.width as u32, size.height as u32)),
        Err(e) => Err(FileParseError::LibraryParseError {
            message: e.to_string(),
        })
    }
}

fn generate_md5(bytes: &[u8]) -> String {
    let output = md5::Md5::digest(bytes);
    format!("{:x}", output)
}

fn parse_fs_content_length(path: &Path) -> Result<u64, FileParseError> {
    fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e|
            FileParseError::IOError(
                path.to_path_buf(),
                e.to_string(),
            ))
}

fn parse_title(path: &Path) -> Result<OsString, FileParseError> {
    let name = path.file_name();
    match name {
        Some(value) => {
            Ok(value.to_os_string())
        }
        None => {
            // Unsure if the name can actually not be present?
            Err(FileParseError::NoFileName(PathBuf::from(path), "No file name".to_string()))
        }
    }
}

fn parse_extension(path: &Path) -> Result<Extension, FileParseError> {
    match path.extension() {
        Some(str) => {
            let extension = match_extension(str);
            if extension != Extension::UNKNOWN {
                Ok(extension)
            } else {
                Err(FileParseError::ParseError(PathBuf::from(path), "Unknown file extension".to_string()))
            }
        }
        None => Err(FileParseError::ParseError(PathBuf::from(path), "No file extension".to_string()))
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
        // .filter(|entry| entry.as_ref().unwrap().file_name() == OsString::from("2e0da20211010_173216.jpg"))
        // .filter(|entry| entry.as_ref().unwrap().file_name() == OsString::from("34e97IMG_8492.MOV"))
        // .take(100)
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>())
}

impl Default for Extension {
    fn default() -> Self { Extension::UNKNOWN }
}

fn convert_library_error_to_file_parse_error(err: FileParseError, path: &Path) -> FileParseError {
    if let FileParseError::LibraryParseError { message } = err {
        FileParseError::ParseError(path.to_path_buf(), message)
    } else {
        err
    }
}
