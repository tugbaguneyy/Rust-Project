fn main() {
    //i choose vector type for 5. item :create a collection with some element ...
    let numbers = vec![1, 4, 3, 4, 5];
    let condition = FilterCondition { field: 4 };
    //stored the filtered values in vec
    let filtered_values = custom_filter(&numbers, &condition);

    println!("Original collection: {:?}", numbers);
    println!("Filtered values: {:?}", filtered_values);
}
struct FilterCondition<T> {
    field: T,
}
//The is_match method determines whether a given item matches the filter condition.
//The PartialEq (Partial Equality) attribute is a trait used in Rust to compare whether two values are equal.
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.field == item
    }
}

fn custom_filter<'a, T>(collection: &'a [T], filter_condition: &FilterCondition<T>) -> Vec<&'a T>
//The operation is executed when T matches the PartialEq value.
where
    T: PartialEq,
{
    let mut result = Vec::new();

    for item in collection {
        if filter_condition.is_match(item) {
            result.push(item);
        }
    }

    result
}


