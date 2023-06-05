use provoit_types::models::users::User;
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

/// Authentication request guard.
/// Must be a parameter of authenticated routes.
///
/// See https://rocket.rs/v0.5-rc/guide/requests/#request-guards
pub struct Auth(pub User);

#[cfg(debug_assertions)]
async fn is_valid(_req: &Request<'_>, _db: Db) -> Option<User> {
    use sha2::{Digest, Sha512};

    Some(User {
        id: 1,
        firstname: "root".to_owned(),
        lastname: "".to_owned(),
        mail: "root@provoit.com".to_owned(),
        passwd: base16ct::lower::encode_string(&Sha512::digest("test")),
        token: Some("".to_owned()),
        token_gentime: Some(chrono::Local::now().naive_local()),
        profile_pic: None,
        smoker: false,
        id_favorite_vehicle: None,
    })
}

#[cfg(not(debug_assertions))]
async fn is_valid(req: &Request<'_>, db: Db) -> Option<User> {
    use std::ops::Sub;

    use chrono::Duration;
    use diesel::prelude::*;
    use provoit_types::schema::users;

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

        if let Ok(user) = user {
            return user;
        }
    }

    None
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = Db::from_request(req).await.unwrap();

        if let Some(user) = is_valid(req, db).await {
            Outcome::Success(Auth(user))
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
