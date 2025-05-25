use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::user::{InsertableUser, User};
use crate::services::user_service;

pub(crate) mod expenses;
pub(crate) mod users;

/**
Ajoute un groupe après avoir vérifié que l'utilisateur est bien le propriétaire du groupe.
*/
#[post("/", format = "application/json", data = "<group>")]
pub fn create_group(group: Json<GroupWithUser>, authenticated_user: &AuthenticatedUser) -> Result<Json<Group>, (Status, String)> {
    let group_with_user = group.into_inner();
    match group_service::create_group(&group_with_user, authenticated_user) {
        Ok(group) => {
            Ok(Json(group))
        }
        Err(status) => Err(status),
    }
}