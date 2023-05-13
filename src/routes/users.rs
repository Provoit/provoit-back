use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use provoit_types::models::{NewUser, User};
use provoit_types::schema::*;

use super::DbResult;
use crate::database::Db;

#[get("/<id>")]
pub async fn read(db: Db, id: u64) -> DbResult<Json<User>> {
    let users: User = db
        .run(move |conn| users::table.find(id as i64).get_result(conn))
        .await?;

    Ok(Json(users))
}

#[get("/")]
pub async fn list(db: Db) -> DbResult<Json<Vec<User>>> {
    let users: Vec<User> = db
        .run(move |conn| users::table.select(users::all_columns).load(conn))
        .await?;

    Ok(Json(users))
}

#[post("/", data = "<user>")]
pub async fn create(db: Db, user: Json<NewUser>) -> DbResult<Created<()>> {
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&*user)
            .execute(conn)
    })
    .await?;

    Ok(Created::new("/"))
}

// #[put("/<id>", data = "<user>")]
// pub async fn update(db: Db, id: u64, user: Json<NewUser>) -> DbResult<()> {
//     db.run(move |conn| {
//         diesel::update(users::table)
//             .set(user)
//             .filter(users::id.eq(id as i64))
//             .execute(conn)
//     })
//     .await?;
//
//     Ok(())
// }

#[delete("/")]
pub async fn delete(db: Db) -> DbResult<()> {
    db.run(move |conn| diesel::delete(users::table).execute(conn))
        .await?;

    Ok(())
}
