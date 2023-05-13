use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};

use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;

#[database("diesel_demo")]
/// Wrapper for a database connection.
/// To be used by route functions.
pub struct Db(diesel::MysqlConnection);

/// Run migrations on rocket build
async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    Db::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            provoit_types::migrations::run_migrations(conn).unwrap();
        })
        .await;

    rocket
}

/// Setup the database for global use
pub fn setup() -> AdHoc {
    AdHoc::on_ignite("Diesel MySQL Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}
