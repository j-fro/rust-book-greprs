use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let search = &args[1];
    let filename = &args[2];

    println!("Searching for {}", search);
    println!("Searching in {}", filename);

    let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong when reading the file");

    println!("With text {}", contents);
}
