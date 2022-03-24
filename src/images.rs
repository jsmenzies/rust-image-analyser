use std::{fmt, fs, io};
use std::fs::{DirEntry, ReadDir};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use image::io::Reader;
use md5::Digest;

use crate::images::FileParseError::FileIsEmpty;

// parse the directory recursively and return a list of all files
pub fn parse_dir_to_metadata(dir: &Path) -> Vec<Metadata> {
    println!("Parsing directory: {}", dir.display());
    let result = fs::read_dir(dir);

    let files = result
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>();

    parse_images(files);

    Vec::new()
}

fn parse_images(paths: Vec<PathBuf>) -> Vec<Metadata> {
    let mut images = Vec::new();

    for path in paths {
        let metadata = Metadata::new(&path);
        match metadata {
            Ok(mut result) => {
                result.attempt_parse();
                result.generate_md5_hash();
                images.push(result);
            }
            Err(e) => println!("{}", e),
        }
    }

    images
}

impl Metadata {
    fn new(path: &Path) -> Result<Self, io::Error> {

        if path.exists() && path.is_file() {
            // let file_name = path.file_name().unwrap().to_str().unwrap();
            // let file_size = path.metadata()?.len();
            // let file_extension = path.extension().unwrap().to_str().unwrap();


            Ok(Self {
                // path: path.to_path_buf(),
                // title: file_name.to_string(),
                // file_size: file_size,
                // file_extension: file_extension.to_string(),
                // file_md5_hash: String::new(),
                // file_parse_error: None,
                path: Path::to_path_buf(path),
                title: path.file_name().unwrap().to_str().unwrap().to_string(),
                extension: read_extension(path),
                ..Metadata::default()
            })
        } else {
            Err(Error::new(ErrorKind::NotFound, "File not found"))
        }
    }

    fn attempt_parse(&mut self) -> Result<(), io::Error> {
        self.parsed = true;
        fs::metadata(&self.path).unwrap();
        Ok(())
    }

    fn generate_md5_hash(&mut self) -> Result<(), io::Error> {
        let vec = fs::read(&self.path)?;
        let output = md5::Md5::digest(vec);
        let hash = format!("{:x}", output);
        self.md5 = hash;
        Ok(())
    }
}

fn read_extension(path: &Path) -> Extension {
    let extension = path.extension();

    match extension {
        Some(ext) => {
            let ext = ext.to_str().unwrap();
            match ext {
                "jpg" => Extension::JPG,
                "png" => Extension::PNG,
                "gif" => Extension::GIF,
                "bmp" => Extension::BMP,
                "webp" => Extension::WEBP,
                "tiff" => Extension::TIFF,
                "tif" => Extension::TIFF,
                "jpeg" => Extension::JPG,
                _ => Extension::UNKNOWN,
            }
        }
        None => Extension::UNKNOWN,
    }
}

#[derive(Debug)]
enum Extension {
    JPG,
    PNG,
    GIF,
    BMP,
    TIFF,
    ICO,
    PSD,
    WEBP,
    UNKNOWN,
}

impl Default for Extension {
    fn default() -> Self { Extension::UNKNOWN }
}

#[derive(Default, Debug)]
pub struct Metadata {
    id: String,
    path: PathBuf,
    title: String,
    extension: Extension,
    parsed: bool,
    md5: String,
    // width: u32,
    // height: u32,
    // content_length: u32,
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

#[derive(Debug)]
enum FileParseError {
    FileIsEmpty,
    FileInaccessible,
    NoMD5,
}
