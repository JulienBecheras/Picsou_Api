use diesel::RunQueryDsl;
use rocket::http::Status;
use crate::models::group_user::InsertableGroupUser;
use crate::schema::groups_users::dsl::groups_users;
use crate::repositories::group_repository::get_group_by_id;
use crate::repositories::user_repository::get_user_by_id;

/*
 * Permet l'ajout d'un ou plusieur utilisateur dans le groupe
 * Verfifie si le status des utilisateurs est valide
 * Verifie si l'utilisateur existe
 * Verifie si le groupe existe
 */
pub fn insert_all_user_group( conn: &mut diesel::PgConnection, group_user_entries: &[InsertableGroupUser] ) -> Result<usize, (Status, String)> {
    let mut total_inserted = 0;
    for group_user in group_user_entries {
        if !(0..=5).contains(&group_user.status) {
            return Err((Status::BadRequest, "Invalid status for the user".to_string()));
        }
        get_user_by_id(&group_user.id_user)
            .map_err(|e| e)?;
        get_group_by_id(&group_user.id_group)
            .map_err(|e| e)?;
        match diesel::insert_into(groups_users).values(group_user.clone()).execute(conn) {
            Ok(count) => total_inserted += count,
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation, _)) => return Err((Status::Conflict, "One or more group_user already exist".to_string())),
            Err(_) => return Err((Status::InternalServerError, "An internal server error occurred while inserting users".to_string())),
        }
    }
    Ok(total_inserted)
}