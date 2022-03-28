use std::{fs, io};
use std::fmt::Debug;
use std::path::{Path, PathBuf};

use crate::error::ApplicationError;
use crate::metadata::Metadata;

#[derive(Debug, Default)]
pub struct Location<> {
    root: PathBuf,
    pub metadata: Vec<Metadata>,
}

pub fn add_location(root_string: String) -> Result<Location, ApplicationError> {
    let path = Path::new(&root_string);
    let root = PathBuf::from(path);

    match verify_dir(path) {
        Ok(metadata) => Ok(Location { root, metadata}),
        Err(e) => Err(ApplicationError::IOError(root, e.to_string())),
    }
}

pub fn verify_dir(dir: &Path) -> Result<Vec<Metadata>, io::Error> {
    println!("Parsing directory: {}", dir.display());

    let vec = fs::read_dir(dir)?
        // .filter(|entry| entry.as_ref().unwrap().file_name() == OsString::from("2e0da20211010_173216.jpg"))
        // .filter(|entry| entry.as_ref().unwrap().file_name() == OsString::from("34e97IMG_8492.MOV"))

        // .filter(|entry| entry.as_ref().unwrap().file_name() == OsString::from("0b510IMG_20211019_114522358_HDR.jpg"))
        // .filter(|entry| entry.as_ref().unwrap().file_name() == OsString::from("2bdd0IMG_20211019_110426132_HDR.jpg"))
        // .filter(|entry| entry.as_ref().unwrap().file_name() == OsString::from("1d0f3VID-20210812-WA0002.mp4"))

        // .take(2)

        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .map(Metadata::new)
        .collect::<Vec<Metadata>>();

    Ok(vec)
}