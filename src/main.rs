use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// use std::collections::HashMap;
use std::collections::HashSet;

fn read_file_into_set(filename: &str, set: &mut HashSet<String>) {
    let path = Path::new(filename);
    let f = File::open(&path).expect("Unable to open file");
    // let f = File::open(filename).expect("Unable to open file");
    let buf = BufReader::new(f);
    for line in buf.lines() {
        set.insert(line.unwrap());
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut medium: HashSet<String> = HashSet::new();
    read_file_into_set("dicts/medium-0.txt", &mut medium);
    if medium.contains("constant") {
        println!("constant is in the dictionary");
    }

    let mut default: HashSet<String> = HashSet::new();
    read_file_into_set("dicts/default-1.txt", &mut default);

    let mut huge: HashSet<String> = HashSet::new();
    read_file_into_set("dicts/huge-2.txt", &mut huge);

    let mut insane: HashSet<String> = HashSet::new();
    read_file_into_set("dicts/insane-3.txt", &mut insane);

    println!("medium: {}", medium.len());
    println!("default: {}", default.len());
    println!("huge: {}", huge.len());
    println!("insane: {}", insane.len());
}
