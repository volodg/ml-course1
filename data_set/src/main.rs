use const_format::concatcp;

const DATA_DIR: &str = "./data";
const SET_DIR: &str = "/dataset";
const DATASET_DIR: &str = concatcp!(DATA_DIR, SET_DIR);

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world! {}", DATASET_DIR);
    Ok(())
}
