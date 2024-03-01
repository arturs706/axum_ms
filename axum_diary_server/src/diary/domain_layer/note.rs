use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub note_id: Uuid,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
    pub note_text: String,
    pub staff_id: Option<Uuid>, // UUID for Staff (replace with appropriate UUID type for your database)
    pub is_private: bool,
    pub notification: bool,
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
}