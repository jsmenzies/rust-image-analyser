extern crate core;

use std::path::Path;

mod calculator;
mod images;
mod combiner;

fn main() {
    let path = Path::new("/Users/james/proc");
    images::parse_dir_to_metadata(path);
}