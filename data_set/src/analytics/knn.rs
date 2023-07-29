use std::fs::read_to_string;
use drawing_commons::TRAINING_CSV;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SampleRecord {
    width: f64,
    height: f64,
    label: String,
}

pub fn knn() {
    let mut rdr = csv::Reader::from_path(TRAINING_CSV).expect("REASON");
    let samples: Vec<SampleRecord> = rdr.deserialize().into_iter().flat_map(|x| x).collect();

    println!("{:?}", samples);
    // for result in rdr.deserialize() {
    //     // Notice that we need to provide a type hint for automatic
    //     // deserialization.
    //     let record: Record = result?;
    //     println!("{:?}", record);
    // }
}
