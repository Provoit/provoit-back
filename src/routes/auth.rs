use chrono::Local;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl};
use provoit_types::models::users::{LoginUser, User};
use provoit_types::schema::users;
use rand;
use rand::distributions::{Alphanumeric, DistString};
use rocket::http::Cookie;
use rocket::response::Responder;
use rocket::serde::json::Json;

use crate::auth::Auth;
use crate::database::Db;

use super::DbResult;

/// Defines the response to the login route.
#[derive(Responder)]
pub enum LoginResponse<'a> {
    #[response(status = 200)]
    Ok((), Cookie<'a>),
    #[response(status = 401)]
    Unauthorized(()),
}

impl<'a> LoginResponse<'a> {
    /// Creates an `ok` `LoginResponse`.
    pub fn ok(token: String) -> LoginResponse<'a> {
        LoginResponse::Ok((), Cookie::build("token", token).finish())
    }
}

/// Defines the response to the logout route.
/// Logout can only send an empty cookie back.
#[derive(Responder)]
pub struct LogoutResponse<'a> {
    inner: (),
    cookie: Cookie<'a>,
}

impl<'a> LogoutResponse<'a> {
    /// Creates a new `LogoutResponse`.
    pub fn new() -> Self {
        Self {
            inner: (),
            cookie: Cookie::build("token", "").finish(),
        }
    }
}

#[post("/login", data = "<login_user>")]
pub async fn login<'a>(db: Db, login_user: Json<LoginUser>) -> DbResult<LoginResponse<'a>> {
    let user: Option<User> = db
        .run(move |conn| {
            users::table
                .select(users::all_columns)
                .filter(users::mail.eq(&*login_user.mail))
                .filter(users::passwd.eq(&*login_user.passwd))
                .first(conn)
                .optional()
        })
        .await?;

    if let Some(user) = user {
        // generate token and return cookie
        let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 255);
        let db_token = token.clone();

        // Adds token and generation time in database
        db.run(move |conn| {
            diesel::update(users::table)
                .set((
                    users::token.eq(db_token),
                    users::token_gentime.eq(Local::now().naive_local()),
                ))
                .filter(users::id.eq(user.id))
                .execute(conn)
        })
        .await?;

        Ok(LoginResponse::ok(token))
    } else {
        Ok(LoginResponse::Unauthorized(()))
    }
}

#[post("/logout")]
pub async fn logout<'a>(db: Db, auth: Auth) -> DbResult<LogoutResponse<'a>> {
    let user = auth.0;

    db.run(move |conn| {
        diesel::update(users::table)
            .set((
                users::token.eq(None::<String>),
                users::token_gentime.eq(None::<chrono::NaiveDateTime>),
            ))
            .filter(users::id.eq(user.id))
            .execute(conn)
    })
    .await?;

    Ok(LogoutResponse::new())
}
