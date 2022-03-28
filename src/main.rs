#![feature(drain_filter)]
extern crate core;

use std::collections::HashMap;

use crate::extension::Extension;
use crate::metadata::Metadata;

mod file;
mod error;

mod location;
mod extension;
mod metadata;
mod imagesize;
mod exif;

fn main() {
    let root = String::from("/Users/james/proc");
    let mut location = location::add_location(root).unwrap();

    for metadata in location.metadata.iter_mut() {
        match file::parse_file_details(&metadata.path) {
            Ok(details) => metadata.update_file_details(Some(details)),
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }

    // Remove mp4/mov for now
    location.metadata
        .drain_filter(|metadata| {
            match &metadata.file_details {
                Some(details) => {
                    if details.extension != Extension::MP4 && details.extension != Extension::MOV {
                        return false;
                    }
                }
                None => return true
            }
            true
        });

    let x: HashMap<Extension, Vec<Metadata>> = location.metadata
        .iter()
        .fold(HashMap::new(),
              |mut acc, metadata| {
                  let key = metadata.clone().file_details.unwrap().extension;
                  acc.entry(key).or_insert(vec![]).push(metadata.clone());
                  acc
              });

    println!("{:?}", x);

// let new: HashMap<String, String> = old.into_iter().map(|(key, value)| {
//     return (key, some_conversion(value));
// }).collect();

    for metadata in location.metadata.iter_mut() {
        match imagesize::parse_using_imagesize(&metadata.path) {
            Ok(details) => metadata.update_image_size_details(Some(details)),
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }

    // for metadata in location.metadata.iter_mut() {
    //     match exif::parse_exif_details(&metadata.path) {
    //         Ok(details) => metadata.update_exif_details(Some(details)),
    //         Err(error) => {
    //             println!("{:?}", error);
    //         }
    //     }
    // }

    print_file_detail_results(&location.metadata);
    print_file_image_results(&location.metadata);
    print_exif_results(&location.metadata);
}

fn print_file_detail_results(metadata: &[Metadata]) {
    let total = metadata.len();
    let err_count = metadata.iter()
        .filter(|metadata| metadata.file_details.is_none())
        .count();

    println!("Total files: {}", total);
    println!("FS error count {}", err_count);
}

fn print_file_image_results(metadata: &[Metadata]) {
    let err_count = metadata.iter()
        .filter(|metadata| metadata.image_size_details.is_none())
        .count();

    println!("Image size error count: {}", err_count);
}

fn print_exif_results(metadata: &[Metadata]) {
    let err_count = metadata.iter()
        .filter(|metadata| metadata.exif_details.is_none())
        .count();

    println!("Exif error count: {}", err_count);
}
