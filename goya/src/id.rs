use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag", content = "id")]
pub enum WordIdentifier {
    Known(usize, String),   // ID, surface_form
    Unknown(usize, String), // ID, surface_form
}
impl WordIdentifier {
    pub fn get_surface(&self) -> &str {
        match self {
            Self::Known(_, surface) => surface,
            Self::Unknown(_, surface) => surface,
        }
    }
}
