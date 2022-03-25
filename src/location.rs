use super::metadata::Metadata;

use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Default)]
pub struct Location {
    root: PathBuf,
    paths: Vec<PathBuf>,
    metadata: Vec<Metadata>,
}

impl Location {
    pub fn new(root: &str) -> Result<Self, io::Error> {
        let path = Path::new(root);
        let root = PathBuf::from(path);
        let paths = verify_dir(path)?;

        Ok(Self {
            root,
            paths,
            metadata: Vec::new(),
        })
    }

    pub fn metadata(&self) -> &[Metadata] {
        &self.metadata
    }
}

pub fn shallow_pass_location(mut location: Location) -> Result<Location, Vec<Vec<io::Error>>> {
    let mut metadata = Vec::new();
    let mut errors = Vec::new();
    for path in &location.paths {
        match Metadata::new(path.clone()) {
            Ok(m) => metadata.push(m),
            Err(e) => errors.push(e),
        }
    }

    if errors.is_empty() {
        Ok(Location {
            metadata,
            ..location
        })
    } else {
        Err(errors)
    }
}

pub fn verify_dir(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    println!("Parsing directory: {}", dir.display());

    Ok(fs::read_dir(dir)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>())
}
