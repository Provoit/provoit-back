use diesel::prelude::*;
use provoit_types::{
    models::vehicles::{NewVehicle, UpdateVehicle, Vehicle},
    schema::vehicles,
};
use rocket::serde::json::Json;

use crate::{auth::Auth, database::Db};

use super::DbResult;

#[get("/<id>")]
pub async fn read(db: Db, id: u64, _auth: Auth) -> DbResult<Json<Vehicle>> {
    let vehicle: Vehicle = db
        .run(move |conn| vehicles::table.find(id).get_result(conn))
        .await?;

    Ok(Json(vehicle))
}

#[post("/", data = "<vehicle>")]
pub async fn create(db: Db, vehicle: Json<NewVehicle>, _auth: Auth) -> DbResult<()> {
    db.run(move |conn| {
        diesel::insert_into(vehicles::table)
            .values(&*vehicle)
            .execute(conn)
    })
    .await?;

    Ok(())
}

#[put("/<id>", data = "<vehicle>")]
pub async fn update(db: Db, id: u64, vehicle: Json<UpdateVehicle>, _auth: Auth) -> DbResult<()> {
    db.run(move |conn| {
        diesel::update(vehicles::table)
            .set(&*vehicle)
            .filter(vehicles::id.eq(id))
            .execute(conn)
    })
    .await?;

    Ok(())
}

#[delete("/<id>")]
pub async fn delete(db: Db, id: u64, _auth: Auth) -> DbResult<()> {
    db.run(move |conn| {
        diesel::delete(vehicles::table)
            .filter(vehicles::id.eq(id))
            .execute(conn)
    })
    .await?;

    Ok(())
}
