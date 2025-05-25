use rocket::serde::json::Json;
use rocket::http::{Cookie, CookieJar, Status};
use serde::Deserialize;
use crate::models::user::User;
use crate::auth::AuthenticatedUser;
use crate::services::session_service;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/", format = "application/json", data = "<login_request>")]
pub fn create_session(
    login_request: Json<LoginRequest>,
    cookies: &CookieJar<'_>
) -> Result<Json<User>, (Status, String)> {
    match session_service::create_session(&login_request.email, &login_request.password) {
        Ok((user, token)) => {
            cookies.add(Cookie::build(token)
                .http_only(true)
                .same_site(rocket::http::SameSite::Lax)
                .build());
            Ok(Json(user))
        }
        Err(status) => Err(status),
    }
}

#[delete("/", format = "application/json")]
pub fn delete_session(authenticated_user: AuthenticatedUser) -> Result<String, (Status, String)> {
    Ok(
        format!("Cannot delete session of userid {} from server, please delete the session from the client", authenticated_user.user_id)
    )
}


