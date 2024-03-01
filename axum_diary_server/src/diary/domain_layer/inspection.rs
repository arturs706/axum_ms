use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Inspection {
    pub inspection_id: Uuid,
    pub start_time: NaiveDate,
    pub end_time: NaiveDate,
    pub letting: String,
    pub inspection_status: String,
    pub inspection_by_staff_id: Option<Uuid>, // Foreign Key referencing User Management Service
    pub inspection_by_supplier_id: Option<Uuid>, // Foreign Key referencing Supplier Service or related table
    pub office_notes: Option<String>,
    pub tenant_confirmed: bool,
    pub notification: bool,
    pub created_at: DateTime<Utc>,
}