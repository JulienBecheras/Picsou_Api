use rocket::serde::json::Json;
use rocket::http::Status;
use serde::Deserialize;
use crate::models::user::InsertableUser;
use diesel::prelude::*;
use crate::auth::AuthenticatedUser;
use crate::services::user_service::{authenticate_user_service, create_user_service, get_user_token_by_id_service, UserToken};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login", format = "application/json", data = "<login_request>")]
pub fn login(login_request: Json<LoginRequest>) -> Result<Json<UserToken>, (Status, String)> {
    match authenticate_user_service(&login_request.email, &login_request.password) {
        Ok(authUser) => {
            Ok(Json(authUser))
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
#[post("/register", format = "application/json", data = "<insertable_user>")]
pub fn register(insertable_user: Json<InsertableUser>) -> Result<Json<UserToken>, (Status, String)> {
    let insertable_user_entity = insertable_user.into_inner();
    match create_user_service(&insertable_user_entity) {
        Ok(authUser) => {
            Ok(Json(authUser))
        }
        Err(status) => Err(status),
    }
}

#[get("/validate")]
pub fn validate(authenticated_user: AuthenticatedUser) -> Result<Json<UserToken>, (Status, String)> {
    return match get_user_token_by_id_service(&authenticated_user.user_id, &authenticated_user.token) {
        Ok(user_token) => {
            Ok(Json(user_token))
        }
        Err(status) => {
            Err(status)
        }
    }
}


