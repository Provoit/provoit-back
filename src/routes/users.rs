use diesel::prelude::*;
use provoit_types::models::vehicles::Vehicle;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use provoit_types::models::users::{NewUser, UpdateUser, User};
use provoit_types::schema::*;

use super::DbResult;
use crate::auth::Auth;
use crate::database::Db;

#[get("/<id>")]
pub async fn read(db: Db, id: u64, _auth: Auth) -> DbResult<Json<User>> {
    let users: User = db
        .run(move |conn| users::table.find(id).get_result(conn))
        .await?;

    Ok(Json(users))
}

/// Gets the currently logged in user
#[get("/me")]
pub async fn current(auth: Auth) -> DbResult<Json<User>> {
    Ok(Json(auth.0))
}

/// Gets the vehicles of the given user
#[get("/<id>/vehicles")]
pub async fn user_vehicles(db: Db, id: u64, _auth: Auth) -> DbResult<Json<Vec<Vehicle>>> {
    let vehicles: Vec<Vehicle> = db
        .run(move |conn| {
            vehicles::table
                .select(vehicles::all_columns)
                .filter(vehicles::id_user.eq(id))
                .load(conn)
        })
        .await?;

    Ok(Json(vehicles))
}

#[get("/")]
pub async fn list(db: Db, _auth: Auth) -> DbResult<Json<Vec<User>>> {
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

#[put("/<id>", data = "<user>")]
pub async fn update(db: Db, id: u64, user: Json<UpdateUser>) -> DbResult<()> {
    db.run(move |conn| {
        diesel::update(users::table)
            .set(&*user)
            .filter(users::id.eq(id))
            .execute(conn)
    })
    .await?;

    Ok(())
}

#[delete("/<id>")]
pub async fn delete(db: Db, id: u64, _auth: Auth) -> DbResult<()> {
    db.run(move |conn| {
        diesel::delete(users::table)
            .filter(users::id.eq(id))
            .execute(conn)
    })
    .await?;

    Ok(())
}
