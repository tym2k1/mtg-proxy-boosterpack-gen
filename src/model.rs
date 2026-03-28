use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub rarity: String,
    pub type_line: Option<String>,
    pub set: String,
    pub colors: Option<Vec<String>>,
    pub image_uris: Option<ImageUris>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUris {
    pub normal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetInfo {
    pub code: String,
    pub name: String,
}
