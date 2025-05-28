use crate::auth::AuthenticatedUser;
use crate::models::group::Group;
use crate::services::group_service::get_all_groups_service;
use rocket::serde::json::Json;
use rocket::http::Status;

#[get("/groups", format = "application/json")]
pub fn get_all_groups(authenticated_user: AuthenticatedUser) -> Result<Json<Vec<Group>>, (Status, String)> {
    match get_all_groups_service(&authenticated_user) {
        Ok(groups) => Ok(Json(groups)),
        Err(e) => Err(e),
    }
}