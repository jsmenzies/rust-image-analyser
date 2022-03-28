use std::ffi::OsStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Extension {
    JPG,
    MP4,
    PNG,
    MOV,
    HEIF,
    UNKNOWN,
}

pub fn match_extension(value: &OsStr) -> Extension {
    match value.to_ascii_lowercase().to_str().unwrap() {
        "jpg" => Extension::JPG,
        "jpeg" => Extension::JPG,
        "mp4" => Extension::MP4,
        "png" => Extension::PNG,
        "mov" => Extension::MOV,
        "heif" => Extension::HEIF,
        _ => Extension::UNKNOWN,
    }
}

impl Default for Extension {
    fn default() -> Self { Extension::UNKNOWN }
}