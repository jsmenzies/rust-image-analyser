#![warn(clippy::pedantic)]

mod calculator;
mod combiner;
mod images;

fn main() {
    let mut location = images::add_location("/Users/james/proc");

    location = images::shallow_pass_location(location);

    location
        .metadata
        .iter()
        .filter(|meta| !meta.errors.is_empty())
        .for_each(|metadata| {
            println!("{:?}", metadata);
        });

    // println!("{:?}", &location);
}
