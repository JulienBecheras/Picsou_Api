use diesel::Connection;
use rocket::http::Status;
use crate::auth::AuthenticatedUser;
use crate::repositories::group_repository::insert_group;
use crate::repositories::group_user_repository::insert_all_user_group;
use projet_picsou_api::establish_connection;
use crate::models::group::{Group, UserWithStatus, GroupWithUser}; // à adapter selon l’emplacement
use crate::models::group_user::InsertableGroupUser;

pub fn create_group(group_with_user: &GroupWithUser, authenticated_user: AuthenticatedUser) -> Result<Group, (Status, String)> {
    match user_is_owner_of_group(&group_with_user.users, authenticated_user) {
        Ok(_) => {
            let conn = &mut establish_connection();
            conn.transaction(|conn| {
                let group = match insert_group(conn, &group_with_user.group) {
                    Ok(group) => group,
                    Err(e) => return Err(e),
                };

                let user_group_entries: Vec<InsertableGroupUser> = group_with_user.users.iter().map(|user| {
                    InsertableGroupUser {
                        id_user: user.id_user,
                        id_group: group.id,
                        status: user.status,
                    }
                }).collect();

                match insert_all_user_group(conn, &user_group_entries) {
                    Ok(_) => Ok(group),
                    Err(e) => Err(e),
                }
            })
            .map_err(|e| (Status::InternalServerError, e.to_string()))
        }
        Err(e) => Err(e),
    }
}
pub fn user_is_owner_of_group(user_with_status: &Vec<UserWithStatus>, authenticated_user: AuthenticatedUser) -> Result<bool, (Status, String)>{
    let mut nb_owner: i32 = 0;
    let mut owner: i32 = -1;
    for user in user_with_status {
        if user.status == 0 {
            nb_owner = nb_owner + 1;
            owner = user.id_user
        }
    }
    if nb_owner == 1 && owner == authenticated_user.user_id {
        Ok(true)
    }else if  (nb_owner == 1) & (owner !=  authenticated_user.user_id) {
        Err((Status::Unauthorized, "You are not the owner of this group".to_string()))
    }else if nb_owner > 1 {
        Err((Status::Unauthorized, "There are too many owners of this group".to_string()))
    }else { Err((Status::Unauthorized, "There is no owner for this group".to_string())) }
}