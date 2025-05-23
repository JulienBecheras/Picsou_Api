use diesel::{QueryDsl, RunQueryDsl};
use rocket::http::Status;
use projet_picsou_api::establish_connection;
use crate::models::user::{InsertableUser, User};
use crate::schema::users::dsl::users;
use diesel::ExpressionMethods;
use crate::schema::users::{email_paypal, first_name, last_name, profil_pict_ref, tel, tel_wero};

pub fn get_user_by_email_repository(email: &str) -> Result<User, (Status, String)> {
    let conn = &mut establish_connection();

    match users.filter(crate::schema::users::email.eq(&email))
        .first::<User>(conn) {
        Ok(user) => Ok(user as User), //If user is found, we return the first element
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, format!("User with email {} does not exist", email))),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn get_user_by_id_repository(id: &i32) -> Result<User, (Status, String)> {
    let conn = &mut establish_connection();

    match users.filter(crate::schema::users::id.eq(&id))
        .first::<User>(conn) {
        Ok(user) => Ok(user as User), //If user is found, we return the first element
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, format!("User with id {} does not exist", id))),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn insert_user_repository(insertable_user: InsertableUser) -> Result<User, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::insert_into(users).values(&insertable_user).get_result::<User>(conn) {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => Err((Status::Conflict, format!("Cannot insert new user because user with email {} already exist", &insertable_user.email))),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn update_user_repository(user: &User) -> Result<User, (Status, String)> {
    let conn = &mut establish_connection();
    let new_user = user.clone();
    match diesel::update(users.filter(crate::schema::users::id.eq(new_user.id)))
        .set((
            first_name.eq(new_user.first_name),
            last_name.eq(new_user.last_name),
            tel.eq(new_user.tel),
            email_paypal.eq(new_user.email_paypal),
            tel_wero.eq(new_user.tel_wero),
            profil_pict_ref.eq(new_user.profil_pict_ref)
        )).get_result::<User>(conn) {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => Err((Status::Conflict, format!("Cannot update user because user with email {} already exist", user.email))),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn delete_user_repository(user_to_delete: &User) -> Result<usize, (Status, String)> {
    let conn = &mut establish_connection();

    match diesel::delete(users.filter(crate::schema::users::id.eq(user_to_delete.id)))
        .execute(conn)
    {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                Ok(rows_affected)
            } else {
                // Si aucune ligne n'a été supprimée, l'utilisateur n'existait probablement pas
                Err((Status::InternalServerError, "The user could not be deleted".to_string()))
            }
        }
        Err(diesel::result::Error::NotFound) => {
            // L'utilisateur avec l'ID spécifié n'a pas été trouvé
            Err((Status::NotFound, format!("User with id {} does not exist", user_to_delete.id)))
        }
        Err(e) => {
            eprintln!("Error deleting user: {:?}", e);
            Err((Status::InternalServerError, format!("Error deleting user: {:?}", e)))
        }
    }
}
