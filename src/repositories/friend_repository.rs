use rocket::http::Status;
use diesel::prelude::*;
use projet_picsou_api::establish_connection;
use crate::models::friend::{Friend, InsertableFriend};
use crate::models::friend_request::{FriendRequest, InsertableFriendRequest};
use crate::schema::friend_requests::dsl as fr_dsl;
use crate::schema::friends::dsl as f_dsl;

// Créer une demande d'amis
pub fn create_friend_request(request: &InsertableFriendRequest) -> Result<FriendRequest, (Status, String)> {
    let conn = &mut establish_connection();
    diesel::insert_into(fr_dsl::friend_requests)
        .values(request)
        .get_result::<FriendRequest>(conn)
        .map_err(|_| (Status::InternalServerError, "Erreur lors de la création de la demande d'amis".to_string()))
}

// Supprimer une demande d'amis
pub fn delete_friend_request(request_id: i32) -> Result<(), (Status, String)> {
    let conn = &mut establish_connection();
    diesel::delete(fr_dsl::friend_requests.filter(fr_dsl::id.eq(request_id)))
        .execute(conn)
        .map_err(|_| (Status::InternalServerError, "Erreur lors de la suppression de la demande d'amis".to_string()))
        .and_then(|count| if count > 0 { Ok(()) } else { Err((Status::NotFound, "Demande d'amis non trouvée".to_string())) })
}

// Créer un lien d’amitié
pub fn create_friend(friend: &InsertableFriend) -> Result<Friend, (Status, String)> {
    let conn = &mut establish_connection();
    diesel::insert_into(f_dsl::friends)
        .values(friend)
        .get_result::<Friend>(conn)
        .map_err(|_| (Status::InternalServerError, "Erreur lors de la création de l'ami".to_string()))
}

// Supprimer un lien d’amitié
pub fn delete_friend(friend_id: i32) -> Result<(), (Status, String)> {
    let conn = &mut establish_connection();
    diesel::delete(f_dsl::friends.filter(f_dsl::id.eq(friend_id)))
        .execute(conn)
        .map_err(|_| (Status::InternalServerError, "Erreur lors de la suppression de l'ami".to_string()))
        .and_then(|count| if count > 0 { Ok(()) } else { Err((Status::NotFound, "Ami non trouvé".to_string())) })
}

// Récupérer toutes les demandes d'amis pour un utilisateur
pub fn get_friend_requests_for_user(user_id: i32) -> Result<Vec<FriendRequest>, (Status, String)> {
    let conn = &mut establish_connection();
    fr_dsl::friend_requests
        .filter(fr_dsl::from_user_id.eq(user_id).or(fr_dsl::to_user_id.eq(user_id)))
        .load::<FriendRequest>(conn)
        .map_err(|_| (Status::InternalServerError, "Erreur lors de la récupération des demandes d'amis".to_string()))
}

// Récupérer tous les amis d'un utilisateur
pub fn get_friends_for_user(user_id: i32) -> Result<Vec<Friend>, (Status, String)> {
    let conn = &mut establish_connection();
    f_dsl::friends
        .filter(f_dsl::user1_id.eq(user_id).or(f_dsl::user2_id.eq(user_id)))
        .load::<Friend>(conn)
        .map_err(|_| (Status::InternalServerError, "Erreur lors de la récupération des amis".to_string()))
}

// Récupérer une demande d'amis par id
pub fn get_friend_request_by_id(request_id: i32) -> Result<FriendRequest, (Status, String)> {
    let conn = &mut establish_connection();
    fr_dsl::friend_requests
        .filter(fr_dsl::id.eq(request_id))
        .first::<FriendRequest>(conn)
        .map_err(|_| (Status::NotFound, "Demande d'amis non trouvée".to_string()))
}

// Récupérer un objet friend par id
pub fn get_friend_by_id(friend_id: i32) -> Result<Friend, (Status, String)> {
    let conn = &mut establish_connection();
    f_dsl::friends
        .filter(f_dsl::id.eq(friend_id))
        .first::<Friend>(conn)
        .map_err(|_| (Status::NotFound, "Ami non trouvé".to_string()))
}