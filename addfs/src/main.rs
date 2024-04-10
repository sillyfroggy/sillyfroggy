extern crate clap;

use std::env;
use std::fs;

fn main() {
    let files: Vec<String> = env::args().collect();
    let file_name = &files[files.len() - 1];
    let files: &[String] = &files[1..files.len() - 1];
    println!("{:?} converting into {file_name}", files);

    let mut new_file_txt = String::from("");
    for f in files {
        new_file_txt = new_file_txt + fs::read_to_string(f).unwrap().as_str() + "\n";
    }
    fs::write(file_name, new_file_txt).expect("Unable to write to file.");
}