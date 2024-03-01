use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
    pub maintenance_id: Uuid,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
    pub maintenance_job: String,
    pub staff_id: Option<Uuid>, // Foreign Key referencing User Management Service
    pub notification_method: String,
    pub created_at: DateTime<Utc>,
}