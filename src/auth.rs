use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};

use crate::database::Db;

/// Number of hours the auth token is valid
#[cfg(not(debug_assertions))]
const TOKEN_VALIDITY: u8 = 24;

/// Authentification request guard.
/// Must be a parameter of authenticated routes.
///
/// See https://rocket.rs/v0.5-rc/guide/requests/#request-guards
pub struct Auth;

#[cfg(debug_assertions)]
async fn is_valid(_req: &Request<'_>, _db: Db) -> bool {
    true
}

#[cfg(not(debug_assertions))]
async fn is_valid(req: &Request<'_>, db: Db) -> bool {
    use std::ops::Sub;

    use chrono::Duration;
    use diesel::prelude::*;
    use provoit_types::{models::users::User, schema::users};

    let cookie = req.cookies().get("token");

    if let Some(cookie) = cookie {
        let token = cookie.value().to_owned();
        let now_minus_validity = chrono::Local::now()
            .naive_local()
            .sub(Duration::hours(TOKEN_VALIDITY as i64));

        let user: Result<Option<User>, diesel::result::Error> = db
            .run(move |conn| {
                users::table
                    .select(users::all_columns)
                    .filter(users::token.eq(token))
                    .filter(users::token_gentime.gt(now_minus_validity))
                    .first(conn)
                    .optional()
            })
            .await;

        user.is_ok() && user.unwrap().is_some()
    } else {
        false
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = Db::from_request(req).await.unwrap();

        if is_valid(req, db).await {
            Outcome::Success(Auth)
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
