use crate::array::MostFrequentElement;
use crate::models::SampleWithFeatures;
use commons::geometry::get_nearest_k;

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

    pub fn predict(&self, point: &[f64]) -> (String, Vec<SampleWithFeatures>) {
        let sample_points = self
            .features
            .clone()
            .into_iter()
            .map(|x| x.point)
            .collect::<Vec<_>>();

        let indices = get_nearest_k(point, &sample_points, self.k);

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
