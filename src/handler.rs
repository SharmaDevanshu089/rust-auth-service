use crate::services::user_service;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use bcrypt::verify;
use bcrypt::{DEFAULT_COST, hash};
use chrono::{Duration, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use serde::Serialize;
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
use crate::auth::Claims;
/* Moved it to Auth
#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}*/

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
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

pub async fn login_handler(Json(payload): Json<LoginPayload>) -> impl axum::response::IntoResponse {
    info!("Login attempt: {:?}", payload);

    // YE UPAR KYO AA RAHA H PTA NAHI
    let user_result = user_service::find_user_by_email(payload.email).await;

    match user_result {
        Ok(user) => {
            info!("User found: {:?}", user);
            (StatusCode::OK, "User found".to_string());
            let verify_result =
                task::spawn_blocking(move || verify(payload.password, &user.password_hash)).await;
            match verify_result {
                Ok(Ok(true)) => {
                    info!("Password verification successful for user: {}", user.email);
                    (StatusCode::OK, "Login successful!".to_string());
                    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

                    // ONE DAY KO EXPIRY HAI
                    let expiration = Utc::now()
                        .checked_add_signed(Duration::days(1))
                        .expect("Failed to create expiration")
                        .timestamp();

                    // CLAIM TOKEN KA
                    let claims = Claims {
                        sub: user.id.to_string(),
                        exp: expiration as usize,
                    };

                    // TOKEN KO JSON ME BHARNA
                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(jwt_secret.as_bytes()),
                    )
                    .unwrap_or_else(|e| {
                        error!("Token encoding error: {}", e);
                        "failed_to_create_token".to_string()
                    });

                    // TOKEN KO RESPONSE ME
                    (StatusCode::OK, Json(TokenResponse { token })).into_response()
                }
                Ok(Ok(false)) => {
                    info!("Password verification failed for user: {}", user.email);
                    (
                        StatusCode::UNAUTHORIZED,
                        "Invalid email or password".to_string(),
                    )
                        .into_response()
                }
                _ => {
                    error!("Password verification task failed");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )
                        .into_response()
                }
            }
        }
        Err(diesel::result::Error::NotFound) => {
            info!("User not found");
            (
                StatusCode::UNAUTHORIZED,
                "Invalid email or password".to_string(),
            )
                .into_response()
        }
        Err(e) => {
            // KUCH GADBADH HAI
            error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
                .into_response()
        }
    }
}
