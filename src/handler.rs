use crate::services::user_service;
use axum::Json;
use axum::http::StatusCode;
use bcrypt::{DEFAULT_COST, hash};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use serde::Deserialize;
use std::env;
use tokio::task;
use tracing::{error, info};

// YE REGISTER WALE OPTION KE LIYE LIKH RAHA HU
// KYOKI REGISTER KE LIYE KEVAL EMAIL PASSWORD CHIYE BAKI SAB KHUD HI NIKAL LENGE
#[derive(Deserialize, Debug)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn register_handler(Json(payload): Json<RegisterPayload>) -> (StatusCode, String) {
    info!("Received new registration: {:?}", payload);

    // ENCRIPT KAR RAHA HU PASSWORD KO CRATE KI MADAD SE
    let hash_result = task::spawn_blocking(move || hash(payload.password, DEFAULT_COST)).await;

    // ERROR HANDLE KAR RHA HAI
    let hashed_password = match hash_result {
        Ok(Ok(hash)) => {
            info!("Password hashed successfully");
            hash
        }
        Ok(Err(e)) => {
            // ENCRIPTION MAI ERROR
            error!("Password hashing error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to hash password".to_string(),
            );
        }
        Err(e) => {
            // PTA NAHI KYA HUA BUT ERROR RETURN KARO ERROR
            error!("Task spawn error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            );
        }
    };
    let new_user = user_service::create_user(payload.email, hashed_password).await;

    info!("New user created: {:?}", new_user);
    // YE EK TEMPRORY HAI ABHI YAHA ISSE DB M DAAL DUNGA

    (StatusCode::CREATED, "User created successfully".to_string())
}

pub async fn login_handler(Json(payload): Json<LoginPayload>) -> (StatusCode, String) {
    info!("Login attempt: {:?}", payload);

    // YE UPAR KYO AA RAHA H PTA NAHI
    let user_result = user_service::find_user_by_email(payload.email).await;

    match user_result {
        Ok(user) => {
            info!("User found: {:?}", user);
            (StatusCode::OK, "User found".to_string())
        }
        Err(diesel::result::Error::NotFound) => {
            info!("User not found");
            (
                StatusCode::UNAUTHORIZED,
                "Invalid email or password".to_string(),
            )
        }
        Err(e) => {
            // KUCH GADBADH HAI
            error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        }
    }
}
