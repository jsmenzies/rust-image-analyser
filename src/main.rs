extern crate core;

mod calculator;
mod images;
mod combiner;

fn main() {
    let root = String::from("/Users/james/proc");

    let mut location = images::add_location(root).unwrap();

    location = images::shallow_pass_location(location);

    let location = images::deep_pass_location(&mut location);

    let errs = location.metadata
        .iter()
        .filter(|meta| !meta.errors.is_empty())
        .map(|meta| println!("{:?}", meta.errors.clone())).count();

    println!("total count/errors: {}/{}", location.metadata.len(), errs);
    println!("hashmap key/value: {}/{}", &location.lookup.keys().len(),
             &location.lookup.values().flatten().count());



}