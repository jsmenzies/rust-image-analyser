use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::error::ApplicationError;

#[derive(Debug, Default, Clone)]
pub struct ExifDetails {
    pub content_length: u64,
}

pub fn parse_exif_details(path: &Path) -> Result<ExifDetails, ApplicationError> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(&file);
    let exif_reader = exif::Reader::new();

    match exif_reader.read_from_container(&mut reader) {
        Ok(result) => {
            // result.fields()
            //     .for_each(|field| {
            //         println!("{}", field.tag);
            //         println!("{}", field.ifd_num);
            //         println!("{}", field.display_value().with_unit(&result));
            //     });
            Ok(ExifDetails {
                content_length: 200,
            })
        }
        Err(e) => {
            Err(ApplicationError::ExifParseError(path.to_path_buf(), e.to_string()))
        }
    }
}
