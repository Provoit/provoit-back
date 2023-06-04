use chrono::Local;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl};
use provoit_types::models::users::{LoginUser, User};
use rand;
use rand::distributions::{Alphanumeric, DistString};
use rocket::http::Cookie;
use rocket::response::Responder;
use rocket::serde::json::Json;

use crate::database::Db;

use super::DbResult;

#[derive(Responder)]
pub enum LoginResponse<'a> {
    #[response(status = 200)]
    Ok((), Cookie<'a>),
    #[response(status = 401)]
    Unauthorized(()),
}

#[post("/login", data = "<login_user>")]
pub async fn login<'a>(db: Db, login_user: Json<LoginUser>) -> DbResult<LoginResponse<'a>> {
    use provoit_types::schema::users;
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

        let cookie = Cookie::build("token", token).finish();

        Ok(LoginResponse::Ok((), cookie))
    } else {
        Ok(LoginResponse::Unauthorized(()))
    }
}
