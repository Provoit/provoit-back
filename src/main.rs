#[macro_use]
extern crate rocket;

mod auth;
mod database;
mod routes;

use rocket_cors::CorsOptions;
use routes::{timings, trips, users, vehicles, version};

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .to_cors()
        .expect("Failed to setup cors");

    rocket::build()
        .attach(database::setup())
        .attach(cors)
        .mount("/", routes![version::version])
        .mount("/", routes![routes::auth::login, routes::auth::logout])
        .mount(
            "/users",
            routes![
                users::read,
                users::current,
                users::user_vehicles,
                users::list,
                users::create,
                users::update,
                users::delete
            ],
        )
        .mount(
            "/vehicles",
            routes![
                vehicles::read,
                vehicles::create,
                vehicles::update,
                vehicles::delete,
            ],
        )
        .mount(
            "/trips",
            routes![
                trips::read,
                trips::search,
                trips::create,
                trips::join,
                trips::update,
                trips::delete
            ],
        )
        .mount("/timings", routes![timings::read])
}
