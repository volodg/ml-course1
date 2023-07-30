use drawing_commons::array::MostFrequentElement;
use drawing_commons::{TESTING_CSV, TRAINING_CSV};
use lazy_static::lazy_static;
use linfa_nn::{distance::*, LinearSearch, NearestNeighbour};
use ndarray::{Array1, Array2};
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
    pub static ref LABEL_PER_ID: HashMap<usize, &'static str> = {
        let mut result = HashMap::new();

        result.insert(0, "car");
        result.insert(1, "fish");
        result.insert(2, "house");
        result.insert(3, "tree");
        result.insert(4, "bicycle");
        result.insert(5, "guitar");
        result.insert(6, "pencil");
        result.insert(7, "clock");

        result
    };
}

fn read_data(file_name: &str) -> (Vec<[f64; 2]>, Vec<usize>) {
    let mut rdr = csv::Reader::from_path(file_name).expect("REASON");

    rdr.deserialize::<SampleRecord>()
        .into_iter()
        .flat_map(|x| x)
        .map(|x| {
            let id = *ID_PER_LABEL.get(x.label.as_str()).expect("");
            ([x.width, x.height], id)
        })
        .unzip()
}

pub fn knn() {
    let xy_train = read_data(TRAINING_CSV);

    let ids = xy_train.1;
    let xy_train: Array2<_> = xy_train.0.into();

    let model = LinearSearch::new().from_batch(&xy_train, L2Dist).expect("");

    let xy_test = read_data(TESTING_CSV);

    let mut correct_count = 0;
    let total_count = xy_test.0.len();

    for (test, expected_id) in xy_test.0.into_iter().zip(xy_test.1) {
        let point: Array1<_> = vec![test[0], test[1]].into();
        let result = model.k_nearest(point.view(), 50).expect("");
        let result = result
            .into_iter()
            .map(|x| x.1)
            .most_frequent_element()
            .expect("");

        if ids[result] == expected_id {
            correct_count += 1;
        }
    }

    println!(
        "ACCURACY: {correct_count}/{total_count} ({:.2}%)",
        correct_count as f64 / total_count as f64 * 100.0
    );
}
