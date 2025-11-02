// yaha auth inset logic hoga
use axum::{
    RequestPartsExt, async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::env;

// HANDLER SE YAHA LAA RAHA HU EASY NESS FOR PROGRAM
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
