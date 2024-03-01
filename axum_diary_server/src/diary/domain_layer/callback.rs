use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Callback {
    pub callback_id: Uuid,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
    pub contact_id: Option<Uuid>, // Foreign Key referencing Contact Service or related table
    pub description: Option<String>,
    pub staff_id: Option<Uuid>,
    pub notification: bool,
    pub created_at: DateTime<Utc>,
}

// Implement any additional functionality or methods for the Callback entity here
