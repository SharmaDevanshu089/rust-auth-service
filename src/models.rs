//YE FILE ME MODEL DIYE H UNHE DEFINE KARNA HAI
// ISSE SAARI TRAITS IMRPOT KAR RAHA HU
use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

// ABI YE STRUCT BANA RAHA HU MERE DATABSE ME SE UTAR KE
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
