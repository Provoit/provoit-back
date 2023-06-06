#[macro_use]
extern crate rocket;

mod auth;
mod database;
mod routes;
mod cors;

use routes::{users, version};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::setup())
        .attach(cors::setup())
        .mount("/", routes![version::version])
        .mount("/", routes![routes::auth::login, routes::auth::logout])
        .mount(
            "/users",
            routes![
                users::read,
                users::list,
                users::create,
                users::update,
                users::delete
            ],
        )
}
