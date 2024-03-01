use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
    pub training_id: Uuid,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
    pub training_title: String,
    pub training_location: Option<String>,
    pub training_description: Option<String>,
    pub lead_trainer_id: Option<Uuid>, // UUID for Lead Trainer (replace with appropriate UUID type for your database)
    pub additional_participants: Option<Vec<String>>,
    pub notification: bool,
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
}
