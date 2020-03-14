#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableFilter<T> {
    pub filter: Option<T>,
    pub sorting: Option<bool>,
}
