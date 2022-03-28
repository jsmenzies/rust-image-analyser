use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use imagesize::ImageType;

use crate::error::ApplicationError;
use crate::extension::Extension;

#[derive(Clone, Debug, Default)]
pub struct ImageSizeDetails {
    pub width: u32,
    pub height: u32,
    pub extension: Extension,
}

pub fn parse_using_imagesize(path: &Path) -> Result<ImageSizeDetails, ApplicationError> {
    let (width, height) = parse_dimension_from_bytes(path)?;
    let extension = parse_file_type_from_bytes(path)?;

    Ok(ImageSizeDetails {
        width,
        height,
        extension,
    })
}

fn parse_dimension_from_bytes(path: &Path) -> Result<(u32, u32), ApplicationError> {
    match imagesize::size(path) {
        Ok(size) => Ok((size.width as u32, size.height as u32)),
        Err(e) => Err(ApplicationError::LibraryParseError(path.to_path_buf(), e.to_string()))
    }
}

fn parse_file_type_from_bytes(path: &Path) -> Result<Extension, ApplicationError> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(&file);
    let mut header = [0; 12];

    reader.read_exact(&mut header).unwrap();

    match imagesize::image_type(header.as_ref()) {
        Ok(format) => {
            match format {
                ImageType::Jpeg => Ok(Extension::JPG),
                ImageType::Png => Ok(Extension::PNG),
                ImageType::Heif => Ok(Extension::HEIF),
                _ => Err(ApplicationError::UnsupportedFileType(path.to_path_buf(),
                                                               "Unsupported file type".to_string()))
            }
        }
        Err(e) => {
            Err(ApplicationError::LibraryParseError(path.to_path_buf(), e.to_string()))
        }
    }
}