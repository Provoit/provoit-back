use chrono::Local;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl};
use provoit_types::models::users::{LoginUser, User};
use provoit_types::schema::users;
use rand;
use rand::distributions::{Alphanumeric, DistString};
use rocket::response::Responder;
use rocket::serde::Serialize;
use rocket::serde::json::Json;

use crate::auth::Auth;
use crate::database::Db;

use super::DbResult;

#[derive(Serialize)]
pub struct UserInfo {
    user: Option<User>,
    token: String
}

/// Defines the response to the login route.
#[derive(Responder)]
pub enum LoginResponse {
    #[response(status = 200)]
    Ok(Json<UserInfo>),
    #[response(status = 401)]
    Unauthorized(()),
}

#[post("/login", data = "<login_user>")]
pub async fn login<'a>(db: Db, login_user: Json<LoginUser>) -> DbResult<LoginResponse> {
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

        Ok(LoginResponse::Ok(Json(UserInfo {
            user: Some(user),
            token
        })))
    } else {
        Ok(LoginResponse::Unauthorized(()))
    }
}

#[post("/logout")]
pub async fn logout(db: Db, auth: Auth) -> DbResult<()> {
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

    Ok(())
}
