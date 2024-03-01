use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffMeeting {
    pub staff_meeting_id: Uuid,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
    pub title: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub lead_staff_id: Option<Uuid>, // UUID for Lead Staff (replace with appropriate UUID type for your database)
    pub additional_attendees: Option<Vec<String>>,
    pub notification: bool,
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
}
