use crate::models::SampleWithFeatures;
use commons::math::Point;
use std::collections::HashMap;

pub struct KNN {
    features: Vec<SampleWithFeatures>,
    k: usize,
}

impl KNN {
    pub fn new(features: &[SampleWithFeatures], k: usize) -> Self {
        Self {
            features: features.to_vec(),
            k,
        }
    }

    pub fn predict(&self, point: &Point) -> (String, Vec<SampleWithFeatures>) {
        let sample_points = self
            .features
            .iter()
            .map(|x| Point {
                x: x.point[0],
                y: x.point[1],
            })
            .collect::<Vec<_>>();

        let indices = point.get_nearest_k(&sample_points, self.k);

        let nearest_samples = indices
            .iter()
            .map(|i| self.features[*i].clone())
            .collect::<Vec<_>>();

        let (_, (_, label)) = nearest_samples.iter().map(|x| x.sample.label.clone()).fold(
            (HashMap::new(), (0, "".to_owned())),
            |(mut map, (frequency, label)), val| {
                let new_frequency = *map
                    .entry(val.clone())
                    .and_modify(|frq| *frq += 1)
                    .or_insert(1);

                if new_frequency > frequency {
                    (map, (new_frequency, val))
                } else {
                    (map, (frequency, label))
                }
            },
        );

        (label, nearest_samples)
    }
}
