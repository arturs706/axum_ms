#![allow(dead_code)]

use super::custom_error_repo::CustomErrors;
use crate::property::domain_layer::property::Property;
use crate::AppState;
use uuid::Uuid;

pub struct PropertyRepository {
    app_state: AppState,
}

impl PropertyRepository {
    pub async fn new() -> Self {
        PropertyRepository {
            app_state: AppState::new().await,
        }
    }
    pub async fn get_all(&self) -> Result<Vec<Property>, String> {
        let records = sqlx::query_as::<_, Property>("SELECT * FROM staff_users")
            .fetch_all(&self.app_state.database.db)
            .await;

        match records {
            Ok(users) => Ok(users),
            Err(e) => Err(e.to_string()),
        }
    }
}
    