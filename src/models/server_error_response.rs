use rocket::http::Status;
use serde::Serialize;

#[derive(Serialize)]
pub struct ServerErrorResponse {
    pub status: Status,
    pub message: String,
}