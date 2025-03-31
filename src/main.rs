#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Bienvenu sur le projet PICSOU !"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}