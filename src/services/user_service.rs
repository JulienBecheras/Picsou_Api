use rocket::http::Status;
use rocket::serde::Serialize;
use crate::models::user::{InsertableUser, User};
use crate::repositories::user_repository::{delete_user_repository, get_user_by_email_repository, get_user_by_id_repository, insert_user_repository, update_user_repository};
use crate::utils::hash::verify_password;
use crate::utils::jwt::create_jwt;

#[derive(Serialize)]
pub struct UserToken {
    pub user: User,
    pub token: String,
}

pub fn get_user_token_by_email_service(email: &str, token: &str) -> Result<UserToken, (Status, String)> {
    match get_user_by_email_repository(&email) {
        Ok(user) => {
            Ok(UserToken {
                user: user.clone(),
                token: token.to_string(),
            })
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn get_user_token_by_id_service(id: &i32, token: &str) -> Result<UserToken, (Status, String)> {
    match get_user_by_id_repository(&id) {
        Ok(user) => {
            Ok(UserToken {
                user: user.clone(),
                token: token.to_string(),
            })
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn get_user_by_email_service(email: &str) -> Result<User, (Status, String)> {
    match get_user_by_email_repository(&email) {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub fn get_user_by_id_service(id: &i32) -> Result<User, (Status, String)> {
    match get_user_by_id_repository(&id) {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub fn authenticate_user_service(email: &str, password: &str) -> Result<UserToken, (Status, String)> {
    match get_user_by_email_repository(&email) {
        Ok(user) => {
            if verify_password(&password, &user.password) {
                // If the password is correct, create a JWT token
                let user_id = user.id;
                let token = create_jwt(&user_id.to_string());
                return get_user_token_by_email_service(&user.email, &token);
            } else {
                Err((Status::Unauthorized,"Invalid password".to_string()))
            }
        }
        Err(e) => Err(e),
    }
}



pub fn create_user_service(insertable_user: &InsertableUser) -> Result<UserToken, (Status, String)> {

    //Hachage du mot de passe
    let hashed_password = crate::utils::hash::hash_password(&insertable_user.password);
    if hashed_password.is_err() {
        return Err((Status::InternalServerError, "Could not hash de password".to_string()))
    }
    let mut new_insertable_user : InsertableUser = insertable_user.clone();
    new_insertable_user.password = hashed_password.unwrap();

    match insert_user_repository(new_insertable_user) {
        Ok(user) => {
            // If the password is correct, create a JWT token
            let token = create_jwt(&user.id.to_string());
            return get_user_token_by_email_service(&user.email, &token);
        }
        Err(e) => Err(e),
    }
}

pub fn update_user_service(user: &User) -> Result<User, (Status, String)> {
    match update_user_repository(user) {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub fn delete_user_service(user: &User) -> Result<usize, (Status, String)> {
    match delete_user_repository(user) {
        Ok(size) => Ok(size),
        Err(e) => Err(e),
    }
}
