use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum PropertyStatus {
    Available,
    LetAgreed,
    Let,
    Inactive,
    UnderValuation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    pub property_id: Uuid,
    pub landlord: Uuid,
    pub secondaryland: Option<String>,
    pub branch: String,
    pub lead_staff: Option<String>,
    pub onthemarket: bool,
    pub status: PropertyStatus,
    pub dateavailable: Option<String>, // Change type to appropriate datetime type if needed
}

// Implement any additional functionality or methods for the Property entity here
