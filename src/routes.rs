use rocket::response::Debug;

pub mod auth;
pub mod trips;
pub mod users;
pub mod vehicles;
pub mod version;
pub mod timings;

/// Result for a route using diesel
type DbResult<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
