use drawing_commons::TRAINING_CSV;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SampleRecord {
    width: f64,
    height: f64,
    label: String,
}

lazy_static! {
    pub static ref ID_PER_LABEL: HashMap<&'static str, usize> = {
        let mut result = HashMap::new();

        result.insert("car", 0);
        result.insert("fish", 1);
        result.insert("house", 2);
        result.insert("tree", 3);
        result.insert("bicycle", 4);
        result.insert("guitar", 5);
        result.insert("pencil", 6);
        result.insert("clock", 7);

        result
    };
}

pub fn knn() {
    let mut rdr = csv::Reader::from_path(TRAINING_CSV).expect("REASON");
    let (x, y): (Vec<_>, Vec<_>) = rdr
        .deserialize::<SampleRecord>()
        .into_iter()
        .flat_map(|x| x)
        .map(|x| {
            let id = *ID_PER_LABEL.get(x.label.as_str()).expect("");
            (vec![x.width, x.height], id)
        })
        .unzip();

    println!("{:?}", x);
    println!("{:?}", y);
    // for result in rdr.deserialize() {
    //     // Notice that we need to provide a type hint for automatic
    //     // deserialization.
    //     let record: Record = result?;
    //     println!("{:?}", record);
    // }
}
