use crate::array::MostFrequentElement;
use crate::models::SampleWithFeatures;
use commons::math::Point;

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

        let label = nearest_samples
            .iter()
            .map(|x| &x.sample.label)
            .most_frequent_element()
            .expect("")
            .clone();

        (label, nearest_samples)
    }
}
