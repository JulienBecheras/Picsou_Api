use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::user::User;

#[get("/<user_id>/expenses")]
pub fn get_all_expenses_with_friends(user_id: i32, _authenticated_user: AuthenticatedUser) -> Result<Json<User>, (Status, String)> {
    Err((Status::NotImplemented, "Not implemented yet".to_string()))
}

#[get("/<user_id>/expenses/<expense_id>")]
pub fn get_all_expenses_with_friends_detailed(user_id: i32, expense_id: i32, _authenticated_user: AuthenticatedUser) -> Result<Json<User>, (Status, String)> {
    Err((Status::NotImplemented, "Not implemented yet".to_string()))
}