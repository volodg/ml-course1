mod file_utils;

use crate::file_utils::read_drawing_data;

struct Sample {
    #[allow(dead_code)]
    id: usize,
    #[allow(dead_code)]
    label: String,
    #[allow(dead_code)]
    student_name: String,
    #[allow(dead_code)]
    student_id: u64,
}

fn main() -> Result<(), std::io::Error> {
    let drawing_data = read_drawing_data()?;

    let samples: Vec<_> = drawing_data
        .iter()
        .zip(1..)
        .flat_map(|(record, id)| {
            record.get_drawings().iter().map(move |(label, _)| Sample {
                id,
                label: label.to_owned(),
                student_name: record.get_student().to_owned(),
                student_id: record.get_session(),
            })
        })
        .collect();

    println!("Valid entries count {:?}", samples.len());

    Ok(())
}
