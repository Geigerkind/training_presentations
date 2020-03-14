#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult<T> {
    pub result: Vec<T>,
    pub num_items: usize,
}
