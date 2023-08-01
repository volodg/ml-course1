use commons::geometry::Point2D;
use drawing_commons::models::SampleWithFeatures;
use web_commons::chart_models::Sample;

pub fn feature_to_chart_sample(feature: SampleWithFeatures) -> Sample {
    Sample {
        id: feature.sample.id,
        group_id: feature.sample.student_id,
        group_name: feature.sample.student_name,
        truth: None,
        label: feature.sample.label,
        point: Point2D {
            x: feature.point[0],
            y: feature.point[1],
        },
    }
}
