use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::server_error_response::ServerErrorResponse;
use crate::models::user::User;
use crate::repositories::user_repository::{get_user_by_email_repository, get_user_by_id_repository};
use crate::routes::auth::LoginRequest;
use crate::services::user_service::{authenticate_user_service, get_user_by_email_service, get_user_by_id_service, get_user_token_by_id_service, update_user_service, UserToken};

#[get("/<id>")]
pub fn get_user_by_id_route(id: i32, authenticated_user: AuthenticatedUser) -> Result<Json<User>, (Status, String)> {
    match get_user_by_id_service(&id) {
            Ok(user) => {
                let mut tmp_user = user.clone();
                tmp_user.password = "".parse().unwrap();
                Ok(Json(tmp_user))
            }
            Err(status) => Err(status),
    }
}

#[get("/email/<email>")]
pub fn get_user_by_email_route(email: &str, authenticated_user: AuthenticatedUser) -> Result<Json<User>, (Status, String)> {
    match get_user_by_email_service(&email) {
        Ok(user) => {
            let mut tmp_user = user.clone();
            tmp_user.password = "".parse().unwrap();
            Ok(Json(tmp_user))
        }
        Err(status) => Err(status),
    }
}

#[put("/", format = "application/json", data = "<new_user>")]
pub fn update_user(new_user: Json<User>, authenticated_user: AuthenticatedUser) -> Result<Json<User>, (Status, String)> {
    if (authenticated_user.user_id != new_user.id) {
        return Err((Status::Forbidden, "You are not allowed to update this user".to_string()));
    }
    match update_user_service(&new_user) {
        Ok(user) => {
            Ok(Json(user))
        }
        Err(status) => Err(status),
    }
}