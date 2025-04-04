use diesel::{QueryDsl, RunQueryDsl};
use rocket::http::Status;
use projet_picsou_api::establish_connection;
use crate::models::user::{InsertableUser, User};
use crate::schema::users::dsl::users;
use diesel::ExpressionMethods;

pub fn get_user_by_email(email: &str) -> Result<User, Status> {
    let conn = &mut establish_connection();

    match users.filter(crate::schema::users::email.eq(&email))
        .first::<User>(conn) {
        Ok(user) => Ok(user as User), //If user is found, we return the first element
        Err(diesel::result::Error::NotFound) => Err(Status::NotFound),
        Err(e) => Err(Status::InternalServerError),
    }
}

pub fn insert_user(insertable_user: InsertableUser) -> Result<User, Status> {
    let conn = &mut establish_connection();

    match diesel::insert_into(users).values(&insertable_user).get_result::<User>(conn) {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            Err(Status::Conflict)
        }
        Err(e) => Err(Status::InternalServerError),
    }
}