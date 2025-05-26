use crate::models::group_user::{GroupUser, InsertableGroupUser};
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

/**
Ajoute un utilisateur dans un groupe
*/
#[post ("/<group_id>/users", format = "application/json", data = "<group_user>")]
pub fn add_user_to_group(group_id: i32, group_user: Json<InsertableGroupUser>, authenticated_user: AuthenticatedUser) -> Result<Json<String>, (Status, String)> {
    let mut user_group = group_user.into_inner();
    user_group.id_group = group_id;
    match group_service::add_user_to_group_service(&group_id, &user_group, &authenticated_user) {
        Ok(group_user) => Ok(Json(group_user)),
        Err((status, message)) => Err((status, message)),
    }
}