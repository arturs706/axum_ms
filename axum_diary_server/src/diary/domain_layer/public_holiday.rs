use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicHoliday {
    pub public_holiday_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub title: String,
    pub description: Option<String>,
    pub notification_time_minutes: i32, // Notification time in minutes
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
}
