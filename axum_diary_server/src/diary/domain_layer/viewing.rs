use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Viewing {
    pub viewing_id: Uuid,
    pub viewing_type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub property_reference: Uuid, // Replace with the appropriate data type for property reference
    pub staff_id: Uuid, // UUID for Staff (replace with appropriate UUID type for your database)
    pub viewing_status: String,
    pub is_second_viewing: bool,
    pub is_virtual_viewing: bool,
    pub arrangements: Option<String>,
    pub feedback: Option<String>,
    pub internal_notes: Option<String>,
    pub follow_up_date: Option<String>,
    pub is_closed: bool,
    pub notification: bool,
    pub created_at: DateTime<Utc>,
}