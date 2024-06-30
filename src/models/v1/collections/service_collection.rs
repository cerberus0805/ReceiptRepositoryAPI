#[derive(Debug, PartialEq)]
pub struct ServiceCollection<T> {
    pub partial_collection: Vec<T>,
    pub total_count: i64
}
