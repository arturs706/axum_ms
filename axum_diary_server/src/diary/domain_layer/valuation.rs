use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Valuation {
    pub valuation_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub property_owner_id: Uuid, // UUID for Property Owner (replace with appropriate UUID type for your database)
    pub location: String,
    pub staff_id: Uuid, // UUID for Staff (replace with appropriate UUID type for your database)
    pub notes: Option<String>,
    pub instruction_status: Option<String>,
    pub follow_up_date: Option<String>,
    pub notification: bool,
    pub created_at: DateTime<Utc>,
}