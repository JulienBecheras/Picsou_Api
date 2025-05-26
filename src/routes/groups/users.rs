use crate::models::group_user::GroupUser;
use rocket::serde::json::Json;
use rocket::http::Status;
use crate::auth::AuthenticatedUser;
use crate::models::user::UserWithStatus;
use crate::services::group_service;

/**
Récupération des utilisateurs d'un groupe
*/
#[get ("/<group_id>/users", format = "application/json")]
pub fn get_all_users_in_group(group_id: i32, authenticated_user: AuthenticatedUser) -> Result<Json<Vec<UserWithStatus>>, (Status, String)> {
    match group_service::get_users_group_service(&group_id, &authenticated_user) {
        Ok(users) => Ok(Json(users)),
        Err((status, message)) => Err((status, message)),
    }
}