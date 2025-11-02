use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use std::env;
use tracing::error;

pub mod user_service {}
