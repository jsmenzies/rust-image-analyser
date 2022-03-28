use std::default::Default;
use std::path::PathBuf;
use crate::exif::ExifDetails;

use crate::file::FileDetails;
use crate::imagesize::ImageSizeDetails;

#[derive(Clone, Default, Debug)]
pub struct Metadata {
    pub path: PathBuf,
    pub file_details: Option<FileDetails>,
    pub image_size_details: Option<ImageSizeDetails>,
    pub exif_details: Option<ExifDetails>,
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

impl Metadata {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }
    
    pub fn update_file_details(&mut self, file_details: Option<FileDetails>) {
        self.file_details = file_details;
    }
    
    pub fn update_image_size_details(&mut self, image_size_details: Option<ImageSizeDetails>) {
        self.image_size_details = image_size_details;
    }

    pub fn update_exif_details(&mut self, exif_details: Option<ExifDetails>) {
        self.exif_details = exif_details;
    }
}