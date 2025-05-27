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
            let mut cookie = Cookie::new("token", token);
            cookie.set_http_only(true);
            cookie.set_same_site(rocket::http::SameSite::Lax);
            cookies.add(cookie);
            Ok(Json(user))
        }
        Err(status) => Err(status),
    }
}

#[delete("/")]
pub fn delete_session(
    authenticated_user: AuthenticatedUser,
    cookies: &CookieJar<'_>
) -> Result<String, (Status, String)> {
    cookies.remove(Cookie::named("token"));
    Ok(format!(
        "Session supprimée pour l'utilisateur {}",
        authenticated_user.user_id
    ))
}


