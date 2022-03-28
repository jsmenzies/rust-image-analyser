use std::ffi::OsStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Extension {
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

pub fn match_extension(value: &OsStr) -> Extension {
    match value.to_ascii_lowercase().to_str().unwrap() {
        "jpg" => Extension::JPG,
        "jpeg" => Extension::JPG,
        "mp4" => Extension::MP4,
        "png" => Extension::PNG,
        "gif" => Extension::GIF,
        "bmp" => Extension::BMP,
        "tiff" => Extension::TIFF,
        "ico" => Extension::ICO,
        "psd" => Extension::PSD,
        "webp" => Extension::WEBP,
        "mov" => Extension::MOV,
        "heif" => Extension::HEIF,
        "jxl" => Extension::JXL,
        _ => Extension::UNKNOWN,
    }
}

impl Default for Extension {
    fn default() -> Self { Extension::UNKNOWN }
}