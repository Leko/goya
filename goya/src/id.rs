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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_surface_known() {
        let surface = String::from("test");
        let id = WordIdentifier::Known(0, surface.to_string());
        assert_eq!(id.get_surface(), surface);
    }

    #[test]
    fn get_surface_unknown() {
        let surface = String::from("test");
        let id = WordIdentifier::Unknown(0, surface.to_string());
        assert_eq!(id.get_surface(), surface);
    }
}
