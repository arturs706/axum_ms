#![allow(dead_code)]

use super::custom_error_repo::CustomErrors;
use crate::landlord::domain_layer::landlord::Landlord;
use crate::AppState;
use uuid::Uuid;

pub struct LandlordRepository {
    app_state: AppState,
}

impl LandlordRepository {
    pub async fn new() -> Self {
        LandlordRepository {
            app_state: AppState::new().await,
        }
    }
    pub async fn get_all(&self) -> Result<Vec<Landlord>, String> {
        let records = sqlx::query_as::<_, Landlord>("SELECT * FROM landlords")
            .fetch_all(&self.app_state.database.db)
            .await;

        match records {
            Ok(users) => Ok(users),
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn get_by_id(&self, landlord_id: Uuid) -> Result<Landlord, String> {
        let record = sqlx::query_as::<_, Landlord>("SELECT * FROM landlords WHERE landlord_id = $1")
            .bind(landlord_id)
            .fetch_one(&self.app_state.database.db)
            .await;
        match record {
            Ok(landlord) => Ok(landlord),
            Err(e) => Err(e.to_string()),
        }
    }

}