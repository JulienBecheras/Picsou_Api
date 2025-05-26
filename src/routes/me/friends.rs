use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::friend::Friend;
use crate::services::friend_service;

// GET /me/friends
#[get("/friends")]
pub fn get_my_friends(user: AuthenticatedUser) -> Result<Json<Vec<Friend>>, Status> {
    friend_service::get_friends_for_user(user.user_id)
        .map(Json)
        .map_err(|e| e.0)
}

// DELETE /me/friends/<friend_id>
#[delete("/friends/<friend_id>")]
pub fn delete_friend(user: AuthenticatedUser, friend_id: i32) -> Result<Status, Status> {
    friend_service::delete_friend(user.user_id, friend_id)
        .map(|_| Status::NoContent)
        .map_err(|e| e.0)
}