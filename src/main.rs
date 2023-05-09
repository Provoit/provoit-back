#[macro_use]
extern crate rocket;

#[get("/version")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![version])
}
