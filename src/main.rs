extern crate core;

mod calculator;
mod images;
mod combiner;

fn main() {
    let root = String::from("/Users/james/proc");

    let mut location = images::add_location(root).unwrap();

    location = images::shallow_pass_location(location);

    location.metadata
        .iter()
        .filter(|meta| !meta.errors.is_empty())
        .for_each(|metadata| {
            println!("{:?}", metadata.errors);
        });

    println!("{:?}", &location.metadata.len());

    location = images::deep_pass_location(location);
    println!("{:?}", &location.metadata.len());

    let errs = location.metadata
        .iter()
        .filter(|meta| !meta.errors.is_empty());

    for err in errs.clone() {
        println!("{:?}", err.errors);
    };

    println!();
    print!("{:?}", errs.count());
    print!("/");
    print!("{:?}", location.metadata.len());
    println!();
}