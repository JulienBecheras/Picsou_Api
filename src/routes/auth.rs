use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rocket::http::Method::Post;
use rocket::http::Status;
use rocket::Response;
use rocket::response::status::Unauthorized;
use serde::{Deserialize, Serialize};
use projet_picsou_api::establish_connection;
use crate::models::user::{InsertableUser, User};
use diesel::prelude::*;
use crate::schema::users::dsl::users;
use crate::schema::users::email;
use crate::auth::AuthenticatedUser;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[post("/login", format = "application/json", data = "<login_request>")]
pub fn login(login_request: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
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

    return if crate::utils::hash::verify_password(&login_request.password, &user.password) {
        //If the password is correct, create a JWT token
        let user_id = user.id;
        let response = LoginResponse {
            user: user.clone(),
            token: crate::utils::jwt::create_jwt(&user_id.to_string()),
        };
        Ok(Json(response))
    } else {
        //If the password is incorrect, return an error
        Err(Status::Unauthorized)
    }
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub tel: String,
    pub rib: String,
    pub email_paypal: String,
    pub tel_wero: String,
    pub profil_pict_ref: String,
    pub password: String,
}
#[derive(Serialize)]
struct RegisterResponse {
    pub new_user: User,
    pub token: String,
}

#[post("/register", format = "application/json", data = "<register_request>")]
pub fn register(register_request: Json<RegisterRequest>) -> Result<Json<RegisterResponse>, Status> {
    let conn = &mut establish_connection();
    let hashed_password = crate::utils::hash::hash_password(&register_request.password);
    if hashed_password.is_err() {
        return Err(Status::InternalServerError)
    }
    let insertable_user = InsertableUser {
        first_name: register_request.first_name.clone(),
        last_name: register_request.last_name.clone(),
        email: register_request.email.clone(),
        tel: register_request.tel.clone(),
        rib: register_request.rib.clone(),
        email_paypal: register_request.email_paypal.clone(),
        tel_wero: register_request.tel_wero.clone(),
        profil_pict_ref: register_request.profil_pict_ref.clone(),
        password: hashed_password.unwrap(),
    };
    let result = diesel::insert_into(users).values(&insertable_user).get_result::<User>(conn);
    if result.is_err() {
        return Err(Status::InternalServerError)
    }
    let result = result.unwrap();
    let user_id = result.id;
    let response = RegisterResponse {
        new_user: result,
        token: crate::utils::jwt::create_jwt(&user_id.to_string()),
    };
    Ok(Json(response))
}


#[get("/validate")]
pub fn validate(user: AuthenticatedUser) -> &'static str {
    "Your token is valid"
}

