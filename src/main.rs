mod models;
mod repositories;
mod routes;
mod schema;
mod services;
mod utils;
mod auth;

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::Request;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: u16,
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: status.code })
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to Picsou Project!"
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .register("/", catchers![default_catcher])
        .mount("/", routes![index])
        .mount("/auth", routes![routes::auth::login, routes::auth::register, routes::auth::validate])
        .mount("/user", routes![routes::user::get_user_by_id_route, routes::user::get_user_by_email_route, routes::user::update_user])
}