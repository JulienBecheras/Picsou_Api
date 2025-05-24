use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::user::User;
use crate::services::user_service;

pub(crate) mod friends;
pub(crate) mod groups;

#[put("/", format = "application/json", data = "<new_user>")]
pub fn update_user(new_user: Json<User>, authenticated_user: AuthenticatedUser) -> Result<Json<User>, (Status, String)> {
    if authenticated_user.user_id != new_user.id {
        return Err((Status::Forbidden, "You are not allowed to update this user".to_string()));
    }
    match user_service::update_user(&new_user) {
        Ok(user) => {
            Ok(Json(user))
        }
        Err(status) => Err(status),
    }
}

#[delete("/", format = "application/json")]
pub fn delete_user(authenticated_user: AuthenticatedUser) -> Result<usize, (Status, String)> {
    match user_service::delete_user(&authenticated_user.user_id) {
        Ok(res) => {
            Ok(res)
        }
        Err(status) => Err(status),
    }
}

#[get("/", format = "application/json")]
pub fn get_user(authenticated_user: AuthenticatedUser) -> Result<User, (Status, String)> {
    match user_service::get_user_by_id(&authenticated_user.user_id) {
        Ok(res) => {
            Ok(res)
        }
        Err(status) => Err(status),
    }
}