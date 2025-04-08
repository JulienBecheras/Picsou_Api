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
use crate::repositories::user_repository::{get_user_by_email, get_user_by_id};
use crate::services::user_service::{authenticate_user, create_user};

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

    match authenticate_user(&login_request.email, &login_request.password) {
        Ok(token) => {
            let user: User = get_user_by_email(&login_request.email)?;
            Ok(Json(LoginResponse {
                user: user.clone(),
                token,
            }))
        }
        Err(status) => Err(status),
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

#[post("/register", format = "application/json", data = "<insertable_user>")]
pub fn register(insertable_user: Json<InsertableUser>) -> Result<Json<RegisterResponse>, Status> {
    let insertable_user_entity = insertable_user.into_inner();
    match create_user(&insertable_user_entity) {
        Ok(token) => {
            Ok(Json(RegisterResponse {
                new_user: get_user_by_email(&insertable_user_entity.email)?,
                token,
            }))
        }
        Err(e) => Err(e),
    }

}


#[get("/validate")]
pub fn validate(authenticated_user: AuthenticatedUser) -> Result<Json<LoginResponse>, Status> {
    let user: User = get_user_by_id(&authenticated_user.user_id)?;

    Ok(Json(LoginResponse {
        user: user.clone(),
        token: authenticated_user.token,
    }))
}

