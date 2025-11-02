use crate::models::{NewUser, Users};
use crate::schema::users;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use std::env;
use tracing::error;

pub mod user_service {
    use crate::models::{NewUser, Users};
    // saare import kar raha hu kyoki tabhi kaam karega (eg me same hai)
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
    use std::env;
    use tracing::error;
    pub async fn create_user(email_in: String, hash_in: String) -> Users {
        // PURNANA DATABASE URL NIKAL LIYA
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut conn = AsyncPgConnection::establish(&db_url)
            .await
            .expect("Failed to connect to database");

        //NAYE USER KE LIYE PAYLOAD
        let new_user = NewUser {
            email: email_in,
            password_hash: hash_in,
        };

        // USING DEISEL FOR SQL QUERY OF INSERT INTO TABLE TABLENAME
        let user = diesel::insert_into(users)
            .values(&new_user)
            // ISSE RESULT VAPIS RETUNR HOGA CONNECTION KA
            .get_result::<Users>(&mut conn)
            .await
            .expect("Error saving new user");

        //USER KO RETURN KARNA HAI
        user
    }
    pub async fn find_user_by_email(email_in: String) -> Result<Users, diesel::result::Error> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut conn = AsyncPgConnection::establish(&db_url)
            .await
            .expect("Failed to connect to database");
        let user = users // `users` comes from the dsl
            .filter(email.eq(email_in)) // This is the WHERE clause
            .first::<Users>(&mut conn) // Get the first result or fail
            .await;
        user
    }
}
