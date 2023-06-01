use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};

pub struct Auth;

#[cfg(debug_assertions)]
fn is_valid(_: &Request<'_>) -> bool {
    true
}

#[cfg(not(debug_assertions))]
fn is_valid(req: &Request<'_>) -> bool {
    todo!()
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if is_valid(req) {
            Outcome::Success(Auth)
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
