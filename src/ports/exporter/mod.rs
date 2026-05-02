use crate::domain::Session;

/// Serialises a `Session` into a target format (JSON for MVP).
pub trait Exporter {
    fn export(&self, session: &Session) -> serde_json::Value;
}
