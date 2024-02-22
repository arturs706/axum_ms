#![allow(dead_code)]

use super::custom_error_repo::CustomErrors;
use crate::user::domain_layer::user::StaffUser;
use crate::AppState;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordVerifier,
};
use uuid::Uuid;

pub struct UserRepository {
    app_state: AppState,
}

impl UserRepository {
    pub async fn new() -> Self {
        UserRepository {
            app_state: AppState::new().await,
        }
    }
    pub async fn get_all(&self) -> Result<Vec<StaffUser>, String> {
        let records = sqlx::query_as::<_, StaffUser>("SELECT * FROM staff_users")
            .fetch_all(&self.app_state.database.db)
            .await;

        match records {
            Ok(users) => Ok(users),
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn get_by_id(&self, user_id: Uuid) -> Result<StaffUser, String> {
        let record = sqlx::query_as::<_, StaffUser>("SELECT * FROM staff_users WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.app_state.database.db)
            .await;
        match record {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn save(&self, user: StaffUser) -> Result<StaffUser, String> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(user.passwd.as_bytes(), &salt)
            .unwrap()
            .to_string();
        let record = sqlx::query_as::<_, StaffUser>("INSERT INTO staff_users (user_id, name, username, mob_phone, passwd, acc_level, status, a_created) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
            .bind(user.user_id)
            .bind(user.name)
            .bind(user.username)
            .bind(user.mob_phone)
            .bind(password_hash)
            .bind(user.acc_level)
            .bind(user.status)
            .bind(user.a_created)
            .fetch_one(&self.app_state.database.db)
            .await;
        match record {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn update(&self, user: StaffUser) -> Result<StaffUser, String> {
        let record = sqlx::query_as::<_, StaffUser>("UPDATE staff_users SET name = $1, username = $2, mob_phone = $3, passwd = $4, acc_level = $5, status = $6, a_created = $7 WHERE user_id = $8 RETURNING *")
            .bind(user.name)
            .bind(user.username)
            .bind(user.mob_phone)
            .bind(user.passwd)
            .bind(user.acc_level)
            .bind(user.status)
            .bind(user.a_created)
            .bind(user.user_id)
            .fetch_one(&self.app_state.database.db)
            .await;
        match record {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn delete(&self, user_id: Uuid) -> Result<(), String> {
        let record = sqlx::query("DELETE FROM staff_users WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.app_state.database.db)
            .await;
        match record {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn login(&self, username: String, passwd: String) -> Result<StaffUser, CustomErrors> {
        let record = sqlx::query_as::<_, StaffUser>(
            "SELECT * FROM staff_users WHERE username = $1 AND passwd = $2",
        )
        .bind(username)
        .fetch_one(&self.app_state.database.db)
        .await;
        match record {
            Ok(user) => {
                let parsed_hash = PasswordHash::new(&user.passwd).unwrap();
                let is_pass_valid = Argon2::default()
                    .verify_password(passwd.as_bytes(), &parsed_hash)
                    .is_ok();
                match is_pass_valid {
                    true => Ok(user),
                    _ => Err(CustomErrors::NotAuthorized),
                }
            }
            Err(e) => {
                println!("Login error: {:?}", e);
                Err(CustomErrors::NotAuthorized)
            }
        }
    }
}
