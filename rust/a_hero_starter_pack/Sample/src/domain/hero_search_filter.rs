use crate::domain::TableFilter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroSearchFilter {
    pub page: u32,
    pub id: TableFilter<u32>,
    pub name: TableFilter<String>,
    pub sub_title: TableFilter<String>,
    pub strength: TableFilter<String>,
    pub weakness: TableFilter<String>,
    pub hero_call: TableFilter<String>,
}