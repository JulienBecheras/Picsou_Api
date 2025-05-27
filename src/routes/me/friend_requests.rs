use rocket::http::Status;
use rocket::serde::json::Json;
use crate::auth::AuthenticatedUser;
use crate::models::friend::{Friend};
use crate::models::friend_request::{DetailedFriendRequest, FriendRequest, InsertableFriendRequest};
use crate::models::user::User;
use crate::services::friend_service;



// POST /me/friends/requests
#[post("/friends/requests", data = "<request>")]
pub fn create_friend_request(user: AuthenticatedUser, request: Json<InsertableFriendRequest>) -> Result<Json<FriendRequest>, Status> {
    let req = request.into_inner();
    if req.from_user_id != user.user_id {
        return Err(Status::Forbidden); // L'utilisateur connecté n'est pas l'émetteur de la demande
    }
    friend_service::create_friend_request(&req)
        .map(Json)
        .map_err(|e| e.0)
}

// GET /me/friends/requests
#[get("/friends/requests")]
pub fn get_my_friend_requests(user: AuthenticatedUser) -> Result<Json<Vec<DetailedFriendRequest>>, Status> {
    friend_service::get_friend_requests_for_user(user.user_id)
        .map(Json)
        .map_err(|e| e.0)
}

// DELETE /me/friends/requests/<request_id>
#[delete("/friends/requests/<request_id>")]
pub fn delete_friend_request(user: AuthenticatedUser, request_id: i32) -> Result<Status, Status> {
    friend_service::delete_friend_request(user.user_id, request_id)
        .map(|_| Status::NoContent)
        .map_err(|e| e.0)
}

// PATCH /me/friends/requests/<request_id>
#[patch("/friends/requests/<request_id>")]
pub fn accept_friend_request(
    user: AuthenticatedUser,
    request_id: i32,
) -> Result<Json<Friend>, Status> {
    friend_service::accept_friend_request(request_id, user.user_id)
        .map(Json)
        .map_err(|e| e.0)
}