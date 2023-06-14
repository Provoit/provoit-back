use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use provoit_types::models::trips::{NewTrip, Trip, UpdateTrip};
use provoit_types::schema::*;

use super::DbResult;
use crate::auth::Auth;
use crate::database::Db;

#[get("/<id>")]
pub async fn read(db: Db, id: u64, _auth: Auth) -> DbResult<Json<Trip>> {
    let trip: Trip = db
        .run(move |conn| trips::table.find(id).get_result(conn))
        .await?;

    Ok(Json(trip))
}

#[get("/")]
pub async fn list(db: Db, _auth: Auth) -> DbResult<Json<Vec<Trip>>> {
    let trips: Vec<Trip> = db
        .run(move |conn| trips::table.select(trips::all_columns).load(conn))
        .await?;

    Ok(Json(trips))
}

#[post("/", data = "<trip>")]
pub async fn create(db: Db, trip: Json<NewTrip>) -> DbResult<Created<()>> {
    db.run(move |conn| {
        diesel::insert_into(trips::table)
            .values(&*trip)
            .execute(conn)
    })
    .await?;

    Ok(Created::new("/"))
}

#[put("/<id>", data = "<trip>")]
pub async fn update(db: Db, id: u64, trip: Json<UpdateTrip>) -> DbResult<()> {
    db.run(move |conn| {
        diesel::update(trips::table)
            .set(&*trip)
            .filter(trips::id.eq(id))
            .execute(conn)
    })
    .await?;

    Ok(())
}

#[delete("/<id>")]
pub async fn delete(db: Db, id: u64, _auth: Auth) -> DbResult<()> {
    db.run(move |conn| {
        diesel::delete(trips::table)
            .filter(trips::id.eq(id))
            .execute(conn)
    })
    .await?;

    Ok(())
}
