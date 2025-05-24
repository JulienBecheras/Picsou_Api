use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::user::{InsertableUser, User};
use crate::services::user_service;

#[get("/friends", format = "application/json")]
pub fn get_all_friends(authenticated_user: AuthenticatedUser) -> Result<String, (Status, String)> {
    Err((Status::NotImplemented, "Not implemented yet".to_string()))
}

#[post("/friends", format = "application/json", data = "<user_id>")]
pub fn create_friendship(user_id: i32) -> Result<String, (Status, String)> {
    Err((Status::NotImplemented, "Not implemented yet".to_string()))
}

#[delete("/friends/<friend_id>", format = "application/json")]
pub fn delete_friendship(friend_id: i32) -> Result<String, (Status, String)> {
    Err((Status::NotImplemented, "Not implemented yet".to_string()))
}