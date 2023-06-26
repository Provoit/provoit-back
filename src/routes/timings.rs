use diesel::prelude::*;
use provoit_types::models::timings::Timing;
use rocket::serde::json::Json;

use provoit_types::schema::*;

use super::DbResult;
use crate::auth::Auth;
use crate::database::Db;

#[get("/<id>")]
pub async fn read(db: Db, id: u64, _auth: Auth) -> DbResult<Json<Timing>> {
    let timing: Timing = db
        .run(move |conn| timings::table.find(id).get_result(conn))
        .await?;

    Ok(Json(timing))
}
