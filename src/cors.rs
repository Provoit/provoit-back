use rocket::{fairing::AdHoc, http::Header};

pub fn setup() -> AdHoc {
    AdHoc::on_response("CORS", |_, res| {
        Box::pin(async move {
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            res.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"));
            res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        })
    })
}
