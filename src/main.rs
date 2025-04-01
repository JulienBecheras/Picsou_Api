mod models;
mod repositories;
mod routes;
mod schema;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcom to picsou project!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/login", routes![routes::auth::login])
}