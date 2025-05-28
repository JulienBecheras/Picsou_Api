use diesel::Connection;
use rocket::http::Status;
use crate::auth::AuthenticatedUser;
use crate::repositories::{group_repository};
use crate::repositories::group_user_repository::{insert_all_user_group, get_users_group, get_owner_group};
use projet_picsou_api::establish_connection;
use crate::models::group::{Group, UserIdWithStatus, GroupWithUser, UpdatableGroup}; // à adapter selon l’emplacement
use crate::models::group_user::{GroupUser, InsertableGroupUser};
use crate::repositories::user_repository::get_users_by_ids;
use crate::models::user::{PublicUser, UserWithStatus};

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
            if is_user_member_of_group(&group.id, authenticated_user.user_id) {
                Ok(group)
            } else {
                Err((Status::Forbidden, "You are not a member of this group".to_string()))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn is_user_member_of_group(group_id: &i32, user_id: i32) -> bool {
    match get_users_group(group_id) {
        Ok(group_users) => {
            for groups_user in group_users {
                if groups_user.id_user == user_id {
                    return true;
                }
            }
            false

        }
        Err(_) => false, 
    }
}

pub fn is_user_member_of_group_get_status(group_id: &i32, user_id: i32) -> Result<i32, (Status, String)> {
    match get_users_group(group_id) {
        Ok(group_users) => {
            for groups_user in group_users {
                if groups_user.id_user == user_id {
                    return Ok(groups_user.status);
                }
            }
            return Ok(-1);

        }
        Err(e) => Err(e)
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
    if is_user_member_of_group(&group.id, authenticated_user.user_id) {
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
            let public_user : PublicUser = PublicUser{
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                rib: user.rib,
                email_paypal: user.email_paypal,
                tel_wero: user.tel_wero,
                profil_pict_ref: user.profil_pict_ref,
            };
            let user_with_status : UserWithStatus = UserWithStatus {
                user : public_user,
                status: group_users.iter()
                    .find(|gu| gu.id_user == user.id)
                    .map(|gu| gu.status)
                    .unwrap_or(5),
                group_user_id: group_users.iter()
                    .find(|gu| gu.id_user == user.id)
                    .map(|gu| gu.id)
                    .unwrap_or(5),
            };
            list_users_with_status.push(user_with_status);
        }
        return Ok(list_users_with_status);

    }else {
        Err((Status::Forbidden, "You are not a member of this group".to_string()))
    }
}pub fn add_user_to_group_service(group_id: &i32, group_user: &InsertableGroupUser, authenticated_user: &AuthenticatedUser) -> Result<String, (Status, String)> {
    let conn = &mut establish_connection();
    let group_users = match get_users_group(group_id) {
        Ok(users) => users,
        Err(e) => return Err(e),
    };
    if user_is_admin_of_group(&group_users, authenticated_user) {
        if group_user.status >= 2 && group_user.status <= 5 {
            match insert_all_user_group(conn,&[group_user.clone()]) {
                Ok(_) => Ok("User added to group successfully".to_string()),
                Err((status, message)) => Err((status, message)),
            }
        }else { return Err((Status::Unauthorized, "The participant you tried to insert get too  many privilege".to_string())) ;}
    } else { return Err((Status::Unauthorized, "You are not an admin of this group".to_string()));}
}

pub fn user_is_admin_of_group(user_with_status: &Vec<GroupUser>, authenticated_user: &AuthenticatedUser) -> bool{
    for user in user_with_status {
        if user.id_user == authenticated_user.user_id && (user.status == 0 || user.status == 1) {
            return true;
        }
    }
    return false;
}

pub fn get_user_by_id_in_group_service(group_id: &i32, user_id: &i32, authenticated_user: &AuthenticatedUser) -> Result<UserWithStatus, (Status, String)> {
    let group = match group_repository::get_group_by_id(group_id) {
        Ok(group) => group,
        Err(e) => return Err(e),
    };
    if is_user_member_of_group(&group.id, authenticated_user.user_id) {
        let group_users = match get_users_group(group_id) {
            Ok(users) => users,
            Err(e) => return Err(e),
        };

        if let Some(group_user) = group_users.iter().find(|gu| gu.id_user == *user_id) {
            let user = match crate::repositories::user_repository::get_user_by_id(user_id) {
                Ok(user) => user,
                Err(e) => return Err(e),
            };
            let public_user = PublicUser {
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                rib: user.rib,
                email_paypal: user.email_paypal,
                tel_wero: user.tel_wero,
                profil_pict_ref: user.profil_pict_ref,
            };
            Ok(UserWithStatus {
                user: public_user,
                status: group_user.status,
                group_user_id: group_user.id,
            })
        } else {
            Err((Status::NotFound, "User not found in this group".to_string()))
        }
    } else {
        Err((Status::Forbidden, "You are not a member of this group".to_string()))
    }
}

pub fn update_user_in_group_service(group_id: &i32, user_id: &i32, status: &i32, authenticated_user: &AuthenticatedUser) -> Result<String, (Status, String)> {
    if is_user_member_of_group(group_id, *user_id){
        let group_users = match get_users_group(&group_id) {
            Ok(users) => users,
            Err(e) => return Err(e),
        };
        if user_is_admin_of_group(&group_users, authenticated_user) {
            if *status >= 2 && *status <= 5 {
                match group_repository::update_user_status_in_group(user_id, &status) {
                    Ok(_) => Ok("User updated in group successfully".to_string()),
                    Err((status, message)) => Err((status, message)),
                }
            } else {
                return Err((Status::Unauthorized, "The participant you tried to update get too many privilege".to_string()));
            }
        } else {
            return Err((Status::Unauthorized, "You are not an admin of this group".to_string()));
        }
    } else { return Err((Status::Unauthorized, "User is not a member of this group".to_string())); }
}

pub fn get_all_groups_service(authenticated_user: &AuthenticatedUser) -> Result<Vec<Group>, (Status, String)> {
    match group_repository::get_all_groups_user_repository(&authenticated_user.user_id) {
        Ok(groups) => Ok(groups),
        Err(e) => Err(e),
    }
}

pub fn modify_group_service(group_id: &i32, group: &UpdatableGroup, authenticated_user: &AuthenticatedUser) -> Result<Group, (Status, String)> {
    let group_users = match get_users_group(group_id) {
        Ok(users) => users,
        Err(e) => return Err(e),
    };
    if user_is_admin_of_group(&group_users, authenticated_user) {
        match group_repository::update_group(group_id, group) {
            Ok(updated_group) => Ok(updated_group),
            Err(e) => Err(e),
        }
    } else {
        Err((Status::Forbidden, "You are not the owner of this group".to_string()))
    }
}