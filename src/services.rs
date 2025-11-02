use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use std::env;
use tracing::error;

pub mod user_service {
    use crate::models::{NewUser, User};
    // saare import kar raha hu kyoki tabhi kaam karega (eg me same hai)
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
    use std::env;
    use tracing::error;
}
