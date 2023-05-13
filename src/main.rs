#[macro_use]
extern crate rocket;

mod database;
mod routes;

use routes::{users, version};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::setup())
        .mount("/", routes![version::version])
        .mount(
            "/users",
            routes![users::read, users::list, users::create, users::delete],
        )
}
