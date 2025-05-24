use rocket::http::Status;
use crate::models::user::{InsertableUser, User};
use crate::repositories::user_repository;
use crate::utils::jwt::create_jwt;

pub fn get_user_by_email(email: &str) -> Result<User, (Status, String)> {
    match user_repository::get_user_by_email(&email) {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub fn get_user_by_id(id: &i32) -> Result<User, (Status, String)> {
    match user_repository::get_user_by_id(&id) {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub fn get_user_essentials_by_id(id: &i32) -> Result<User, (Status, String)> {
    match get_user_by_id(&id) {
        Ok(user) => {
            Ok(hide_personal_data(&user))
        }
        Err(e) => Err(e),
    }
}

pub fn get_user_essentials_by_email(email: &str) -> Result<User, (Status, String)> {
    match get_user_by_email(&email) {
        Ok(user) => {
            Ok(hide_personal_data(&user))
        }
        Err(e) => Err(e),
    }
}

fn hide_personal_data(user: &User) -> User {
    let mut tmp_user = user.clone();
    tmp_user.password = "".parse().unwrap();
    tmp_user.tel = "".parse().unwrap();
    tmp_user
}



pub fn create_user(insertable_user: &InsertableUser) -> Result<(User, String), (Status, String)> {

    //Hachage du mot de passe
    let hashed_password = crate::utils::hash::hash_password(&insertable_user.password);
    if hashed_password.is_err() {
        return Err((Status::InternalServerError, "Could not hash de password".to_string()))
    }
    let mut new_insertable_user : InsertableUser = insertable_user.clone();
    new_insertable_user.password = hashed_password.unwrap();

    match user_repository::insert_user(new_insertable_user) {
        Ok(user) => {
            let token = create_jwt(&user.id.to_string());
            Ok((user, token.to_string()))
        }
        Err(e) => Err(e),
    }
}

pub fn update_user(user: &User) -> Result<User, (Status, String)> {
    match user_repository::update_user(user) {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub fn delete_user(user_id: &i32) -> Result<usize, (Status, String)> {
    match user_repository::delete_user(user_id) {
        Ok(size) => Ok(size),
        Err(e) => Err(e),
    }
}
