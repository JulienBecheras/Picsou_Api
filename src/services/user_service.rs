use rocket::http::Status;
use crate::models::user::{InsertableUser, User};
use crate::repositories::user_repository::{get_user_by_email, insert_user};
use crate::utils::hash::verify_password;
use crate::utils::jwt::create_jwt;

pub fn authenticate_user(email: &str, password: &str) -> Result<String, Status> {
    match get_user_by_email(&email) {
        Ok(user) => {
            if verify_password(&password, &user.password) {
                // If the password is correct, create a JWT token
                let user_id = user.id;
                let token = create_jwt(&user_id.to_string());
                Ok(token)
            } else {
                Err(Status::Unauthorized)
            }
        }
        Err(e) => Err(e),
    }
}

pub fn create_user(insertable_user: &InsertableUser) -> Result<String, Status> {

    //Hachage du mot de passe
    let hashed_password = crate::utils::hash::hash_password(&insertable_user.password);
    if hashed_password.is_err() {
        return Err(Status::InternalServerError)
    }
    let mut new_insertable_user : InsertableUser = insertable_user.clone();
    new_insertable_user.password = hashed_password.unwrap();

    match insert_user(new_insertable_user) {
        Ok(user) => {
            // If the password is correct, create a JWT token
            let token = create_jwt(&user.id.to_string());
            Ok(token)
        }
        Err(e) => Err(e),
    }
}