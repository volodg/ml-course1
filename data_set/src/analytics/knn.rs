use std::fs::read_to_string;
use drawing_commons::TRAINING_CSV;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn knn() {
    let lines = read_lines(TRAINING_CSV);
    println!("{:?}", lines);
}
