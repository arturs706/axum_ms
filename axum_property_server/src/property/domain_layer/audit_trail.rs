use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditTrail {
    pub property_audi_id: Uuid,
    pub property_id: Uuid,
    pub audit_when: Option<DateTime<Utc>>, // Change type to appropriate datetime type if needed
    pub audit_user: String,
    pub audit_description: Option<String>,
}
