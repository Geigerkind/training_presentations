#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    pub id: u32,
    pub name: String,
    pub sub_title: Option<String>,
    pub strength: String,
    pub weakness: Option<String>,
    pub hero_call: Option<String>
}