#[derive(serde::Deserialize)]
pub struct Localization {
    #[serde(rename = "LocalizedNames")]
    pub localized_names: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LocalizedDescriptions")]
    pub localized_descriptions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UniqueName")]
    pub unique_name: String,
    #[serde(rename = "Index")]
    pub id: String,
}
