use rocket::http::Status;
use crate::models::user::User;
use crate::services::user_service;
use crate::utils::hash::verify_password;
use crate::utils::jwt::create_jwt;

pub fn create_session(email: &str, password: &str) -> Result<(User, String), (Status, String)> {
    match user_service::get_user_by_email(&email) {
        Ok(user) => {
            if verify_password(&password, &user.password) {
                // If the password is correct, create a JWT token
                let user_id = user.id;
                let token = create_jwt(&user_id.to_string());
                Ok((user, token.to_string()))
            } else {
                Err((Status::Unauthorized,"Invalid password".to_string()))
            }
        }
        Err(e) => Err(e),
    }
}