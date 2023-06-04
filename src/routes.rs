use rocket::response::Debug;

pub mod auth;
pub mod users;
pub mod version;

/// Result for a route using diesel
type DbResult<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
