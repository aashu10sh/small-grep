use small_grep::{query, read_file, Configuration};
use std::{env::args, process};
fn main() {
    let arguments: Vec<String> = args().collect();
    let config = Configuration::new(&arguments).unwrap_or_else(|error| {
        eprintln!("Problem Parsing Arguments| {error}");
        process::exit(1);
    });
    let contents = read_file(config.filename).unwrap_or_else(|error| {
        eprintln!("Problem Reading File| {error}",);
        process::exit(1);
    });
    let results = query(&contents, config.query);

    for result in results {
        println!("{}|{} ", result.location, result.text);
    }
}
