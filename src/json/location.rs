#[derive(serde::Deserialize)]
pub struct Location {
    #[serde(rename = "Index")]
    pub id: String,
    #[serde(rename = "UniqueName")]
    pub name: String,
}
