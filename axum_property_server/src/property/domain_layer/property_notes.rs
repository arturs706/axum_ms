use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyNotes {
    pub property_note_id: Uuid,
    pub property_id: Uuid,
    pub note: String,
}