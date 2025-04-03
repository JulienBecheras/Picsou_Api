mod models;
mod repositories;
mod routes;
mod schema;
mod services;
mod utils;
mod auth;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Picsou Project!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/auth", routes![routes::auth::login, routes::auth::register, routes::auth::validate])
}