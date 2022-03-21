use std::{fmt, fs, io};
use std::fs::{DirEntry, ReadDir};
use std::io::Error;
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
    for path in paths {
        let mut metadata = Metadata::new(&path);
        metadata.attempt_parse();
        let result = metadata.generate_md5_hash();

        if let Err(ref result) = result {
            println!("{:?}", result);
        }
    }

    Vec::new()
}

impl Metadata {
    fn new(path: &Path) -> Self {
        Self {
            path: Path::to_path_buf(path),
            title: path.file_name().unwrap().to_str().unwrap().to_string(),
            extension: read_extension(path),
            ..Metadata::default()
        }
    }

    fn attempt_parse(&mut self) -> Result<(), io::Error> {
        self.parsed = true;
        Ok(())
    }

    fn generate_md5_hash(&mut self) -> Result<(), FileParseError> {
        let metadata = fs::metadata(&self.path);

        match metadata {
            Ok(metadata) => {
                if metadata.len() == 0 {
                    return Err(FileIsEmpty);
                }

                let content = fs::read(&self.path);

                match content {
                    Ok(content) => {

                        println!("{:x}", md5::Md5::digest(&content));

                    }
                    Err(_error) => {
                        return Err(FileParseError::NoMD5);
                    }
                }
            }
            Err(_e) => {
                return Err(FileParseError::FileInaccessible);
            }
        }

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
