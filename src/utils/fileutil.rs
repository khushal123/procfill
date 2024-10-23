use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    let mut contents: Vec<String> = Vec::new();
    for line in reader.lines() {
        contents.push(line.unwrap());
    }
    contents
}
