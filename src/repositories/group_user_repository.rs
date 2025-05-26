use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::sql_types::Integer;
use rocket::http::Status;
use projet_picsou_api::establish_connection;
use crate::models::group_user::{GroupUser, InsertableGroupUser};
use crate::schema::groups_users::dsl::groups_users;
use crate::repositories::user_repository::get_user_by_id;
use diesel::QueryDsl;
use diesel::SelectableHelper;
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
        match diesel::insert_into(groups_users).values(group_user.clone()).execute(conn) {
            Ok(count) => total_inserted += count,
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation, _)) => return Err((Status::Conflict, "One or more group_user already exist".to_string())),
            Err(_) => return Err((Status::InternalServerError, "An internal server error occurred while inserting users".to_string())),
        }
    }
    Ok(total_inserted)
}

pub fn get_owner_group (group_id: &i32) -> Result<GroupUser, (Status, String)>{
    let conn = &mut establish_connection();
    match diesel::sql_query("SELECT * FROM groups_users WHERE id_group = $1 AND status = $2").bind::<Integer, _>(group_id).bind::<Integer, _>(0).get_result(conn) {
        Ok(owner) => Ok(owner),
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, format!("GroupUser with id {} does not exist", group_id))),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}

pub fn get_users_group (group_id: &i32) -> Result<Vec<GroupUser>, (Status, String)>{
    let conn = &mut establish_connection();
    match groups_users
        .filter(crate::schema::groups_users::id_group.eq(group_id))
        .select(GroupUser::as_select())
        .load(conn) {
        Ok(group_users) => Ok(group_users), //If user is found, we return the first element
        Err(diesel::result::Error::NotFound) => Err((Status::NotFound, format!("There is no GroupUser for the group with id {} ", group_id))),
        Err(_) => Err((Status::InternalServerError, "An internal server error occurred while querying the database".to_string())),
    }
}