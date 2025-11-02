use axum::Json;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use serde::Deserialize;
use std::env;
use tracing::{error, info};

// YE REGISTER WALE OPTION KE LIYE LIKH RAHA HU
// KYOKI REGISTER KE LIYE KEVAL EMAIL PASSWORD CHIYE BAKI SAB KHUD HI NIKAL LENGE
#[derive(Deserialize, Debug)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}
