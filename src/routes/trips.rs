use diesel::{insert_into, prelude::*};
use provoit_types::models::creation::CreateTrip;
use provoit_types::models::trip_road_types::NewTripRoadType;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use provoit_types::models::trips::{Trip, UpdateTrip};
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

#[post("/", data = "<data>")]
pub async fn create(db: Db, mut data: Json<CreateTrip>) -> DbResult<Created<()>> {
    db.run(|conn| {
        MysqlConnection::transaction(conn, move |conn| {
            diesel::insert_into(timings::table)
                .values(&data.start_timing)
                .execute(conn)?;
            diesel::insert_into(timings::table)
                .values(&data.end_timing)
                .execute(conn)?;

            let ids: Vec<u64> = timings::table
                .select(timings::id)
                .order(timings::id.desc())
                .limit(2)
                .load(conn)?;

            data.trip.id_end_timing = *ids.first().expect("End timing id should exist");
            data.trip.id_start_timing = *ids.get(1).expect("Start timing id should exist");

            diesel::insert_into(trips::table)
                .values(&data.trip)
                .execute(conn)?;

            let id_trip: u64 = trips::table
                .select(trips::id)
                .order(trips::id.desc())
                .first(conn)?;

            let trip_road: Vec<NewTripRoadType> = data
                .road_types
                .iter()
                .map(|i| NewTripRoadType {
                    id_trip,
                    id_road_type: *i,
                })
                .collect();

            diesel::insert_into(trip_road_types::table)
                .values(&trip_road)
                .execute(conn)?;

            diesel::result::QueryResult::Ok(())
        })
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
