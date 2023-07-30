use std::collections::HashMap;
use std::hash::Hash;

pub trait MostFrequentElement<A> {
    fn most_frequent_element(self) -> Option<A>;
}

impl<I, A> MostFrequentElement<A> for I
where
    I: Iterator<Item = A>,
    A: Eq + Hash + Clone,
{
    fn most_frequent_element(self) -> Option<A> {
        let (_, (_, result)) = self.fold(
            (HashMap::new(), (0, None)),
            |(mut map, (frequency, label)), val| {
                let new_frequency = *map
                    .entry(val.clone())
                    .and_modify(|frq| *frq += 1)
                    .or_insert(1);

                if new_frequency > frequency {
                    (map, (new_frequency, Some(val)))
                } else {
                    (map, (frequency, label))
                }
            },
        );
        result
    }
}
