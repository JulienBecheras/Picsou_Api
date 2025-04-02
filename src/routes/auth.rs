
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rocket::http::Method::Post;
use rocket::http::Status;
use rocket::Response;
use rocket::response::status::Unauthorized;
use serde::Deserialize;
use projet_picsou_api::establish_connection;
use crate::models::user::User;
use crate::schema::users::dsl::users;
use crate::schema::users::email;
use diesel::prelude::*;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login", format = "application/json", data = "<login_request>")]
pub fn login(login_request: Json<LoginRequest>) -> Result<String, Status> {
    let conn = &mut establish_connection();
    //Getting the user from the database
    let user_result : Result<Vec<User>, diesel::result::Error> = users.filter(email.eq(&login_request.email)).limit(1).load(conn);

    if user_result.is_err() {
        return Err(Status::InternalServerError)
    }
    let user_list = user_result.unwrap();
    if user_list.is_empty() {
        //If the user is not found, return an error
        return Err(Status::NotFound)
    }
    let user = user_list.first().unwrap();

    return if crate::utils::hash::verify_password(&user.password, &login_request.password) {
        //If the password is correct, create a JWT token
        let token = crate::utils::jwt::create_jwt(&user.email);
        Ok(token)
    } else {
        //If the password is incorrect, return an error
        Err(Status::Unauthorized)
    }



}