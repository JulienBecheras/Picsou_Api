use diesel::Connection;
use rocket::http::Status;
use crate::auth::AuthenticatedUser;
use crate::repositories::group_repository;
use crate::repositories::group_user_repository::{insert_all_user_group, get_users_group, get_owner_group};
use projet_picsou_api::establish_connection;
use crate::models::group::{Group, UserIdWithStatus, GroupWithUser}; // à adapter selon l’emplacement
use crate::models::group_user::{InsertableGroupUser};
use crate::repositories::user_repository::get_users_by_ids;
use crate::models::user::UserWithStatus;

pub fn create_group(group_with_user: &GroupWithUser, authenticated_user: &AuthenticatedUser) -> Result<Group, (Status, String)> {
    match user_is_owner_of_group(&group_with_user.users, authenticated_user) {
        Ok(_) => {
            let conn = &mut establish_connection();
            conn.transaction(|conn| {
                let group = match group_repository::insert_group_transac(conn, &group_with_user.group) {
                    Ok(group) => group,
                    Err(_) => return Err(diesel::result::Error::RollbackTransaction),
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
                    Err(_) => Err(diesel::result::Error::RollbackTransaction),
                }
            })
            .map_err(|e| (Status::InternalServerError, e.to_string()))
        }
        Err(e) => Err(e),
    }
}
pub fn user_is_owner_of_group(user_with_status: &Vec<UserIdWithStatus>, authenticated_user: &AuthenticatedUser) -> Result<bool, (Status, String)>{
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

pub fn get_group_by_id(group_id: &i32, authenticated_user: &AuthenticatedUser) -> Result<Group, (Status, String)> {
    match crate::repositories::group_repository::get_group_by_id(group_id) {
        Ok(group) => {
            // Here you would typically check if the authenticated user is a member of the group
            // This is a placeholder for that logic
            if is_user_member_of_group(&group, authenticated_user) {
                Ok(group)
            } else {
                Err((Status::Forbidden, "You are not a member of this group".to_string()))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn is_user_member_of_group(group: &Group, authenticated_user: &AuthenticatedUser) -> bool {
    match get_users_group(&group.id) {
        Ok(group_users) => {
            for groups_user in group_users {
                if groups_user.id_user == authenticated_user.user_id {
                    return true;
                }
            }
            return false;

        }
        Err(_) => return false, // If we can't find the owner, assume not a member
    }
}

pub fn delete_group(group_id: &i32, authenticated_user: &AuthenticatedUser) -> Result<String, (Status, String)> {
    match group_repository::get_group_by_id(group_id) {
        Ok(_) => {
            if user_is_owner_of_group_bd(group_id, authenticated_user){
                match group_repository::delete_group(group_id) {
                    Ok(_) => Ok("Group deleted successfully".to_string()),
                    Err(e) => Err(e),
                }
            } else {
                Err((Status::Forbidden, "You are not the owner of this group".to_string()))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn user_is_owner_of_group_bd(group_id: &i32, authenticated_user: &AuthenticatedUser) -> bool {
    match get_owner_group(group_id){
        Ok(group_user) => {
            if group_user.id_user == authenticated_user.user_id { return true; }
            else { return false; }
        }
        Err(_) => return false,
    }
}

pub fn get_users_group_service(group_id: &i32, authenticated_user: &AuthenticatedUser) -> Result<Vec<UserWithStatus>, (Status, String)> {
    let group = match group_repository::get_group_by_id(group_id) {
        Ok(group) => group,
        Err(e) => return Err(e),
    };
    if is_user_member_of_group(&group, authenticated_user) {
        let group_users = match get_users_group(group_id) {
            Ok(users) => users,
            Err(e) => return Err(e),
        };

        let users = match get_users_by_ids(&group_users.iter().map(|gu| gu.id_user).collect()){
            Ok(users) => users,
            Err(e) => return Err(e),
        };

        let mut list_users_with_status: Vec<UserWithStatus> = Vec::new();
        for user in users {
            let user_with_status : UserWithStatus = UserWithStatus {
                user : user.clone(),
                status: group_users.iter()
                    .find(|gu| gu.id_user == user.id)
                    .map(|gu| gu.status)
                    .unwrap_or(5), // 1 si non trouvé, à adapter selon ta logique              , // Default status to 1 if not found
            };
            list_users_with_status.push(user_with_status);
        }
        return Ok(list_users_with_status);

    }else {
        Err((Status::Forbidden, "You are not a member of this group".to_string()))
    }
}