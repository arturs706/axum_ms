use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Appointment {
    pub appointment_id: Uuid,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
    pub app_type: AppointmentType,
    pub title: String,
    pub location: Option<String>,
    pub property_id: Option<Uuid>, // Foreign Key referencing Property Service
    pub description: Option<String>,
    pub staff_id: Option<Uuid>, // Foreign Key referencing User Management Service
    pub additional_attendees: Vec<String>,
    pub is_private: bool,
    pub notification: bool,
    pub is_recurring: bool,
    pub recurring_info: Option<String>, // JSON to store recurring information if applicable
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AppointmentType {
    MoveIn,
    MoveOut,
    Photos,
    Renewal,
    RentalViewing,
    TenancyAgreementSigning,
}