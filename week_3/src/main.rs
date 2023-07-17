
//? struct for FilterCondition
struct FilterCondition<T> {
    condition: T,
}



impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.condition
    }
}

fn custom_filter<T>(collection: &[T], filter: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    collection
        .iter()
        .filter(|item| filter.is_match(item))
        .cloned()
        .collect()
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5];
    let filter_condition = FilterCondition { condition: 3 };

    let filtered_result = custom_filter(&collection, &filter_condition);

    println!("Filtered Result: {:?}", filtered_result);
}