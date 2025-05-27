use rocket::http::Status;
use diesel::prelude::*;
use projet_picsou_api::establish_connection;
use crate::models::friend::{DetailedFriend, Friend, InsertableFriend};
use crate::models::friend_request::{DetailedFriendRequest, FriendRequest, InsertableFriendRequest};
use crate::models::user::User;
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
pub fn get_friend_requests_for_user(user_id: i32) -> Result<Vec<DetailedFriendRequest>, (Status, String)> {
    use crate::schema::friend_requests::dsl as fr_dsl;
    use crate::schema::users::dsl as users_dsl;
    let conn = &mut establish_connection();

    // Récupère les demandes avec le from_user
    let requests = fr_dsl::friend_requests
        .inner_join(users_dsl::users.on(users_dsl::id.eq(fr_dsl::from_user_id)))
        .filter(fr_dsl::to_user_id.eq(user_id))
        .select((
            fr_dsl::id,
            fr_dsl::from_user_id,
            fr_dsl::to_user_id,
            users_dsl::users::all_columns(),
            fr_dsl::created_at,
            fr_dsl::updated_at,
        ))
        .load::<(i32, i32, i32, User, chrono::NaiveDateTime, chrono::NaiveDateTime)>(conn)
        .map_err(|_| (Status::InternalServerError, "Erreur lors de la récupération des demandes d'amis".to_string()))?;

    // Pour chaque demande, on récupère le to_user
    let mut detailed_requests = Vec::new();
    for (id, from_user_id, to_user_id, from_user, created_at, updated_at) in requests {
        let to_user = users_dsl::users
            .filter(users_dsl::id.eq(to_user_id))
            .first::<User>(conn)
            .map_err(|_| (Status::InternalServerError, "Erreur lors de la récupération du destinataire".to_string()))?;
        detailed_requests.push(DetailedFriendRequest {
            id,
            from_user,
            to_user,
            created_at,
            updated_at,
        });
    }
    Ok(detailed_requests)
}
// Récupérer tous les amis d'un utilisateur
pub fn get_friends_for_user(user_id: i32) -> Result<Vec<DetailedFriend>, (Status, String)> {
    use crate::schema::users::dsl as users_dsl;
    let conn = &mut establish_connection();

    // Cas où l'utilisateur est user1
    let friends1 = f_dsl::friends
        .inner_join(users_dsl::users.on(users_dsl::id.eq(f_dsl::user2_id)))
        .filter(f_dsl::user1_id.eq(user_id))
        .select((
            f_dsl::id,
            f_dsl::user1_id,
            f_dsl::user2_id,
            f_dsl::created_at,
            f_dsl::updated_at,
            users_dsl::users::all_columns(), // user2
        ))
        .load::<(i32, i32, i32, chrono::NaiveDateTime, chrono::NaiveDateTime, User)>(conn)
        .map_err(|e| (Status::InternalServerError, format!("Erreur lors de la récupération des amis: {}", e)))?;

    // Cas où l'utilisateur est user2
    let friends2 = f_dsl::friends
        .inner_join(users_dsl::users.on(users_dsl::id.eq(f_dsl::user1_id)))
        .filter(f_dsl::user2_id.eq(user_id))
        .select((
            f_dsl::id,
            f_dsl::user1_id,
            f_dsl::user2_id,
            f_dsl::created_at,
            f_dsl::updated_at,
            users_dsl::users::all_columns(), // user1
        ))
        .load::<(i32, i32, i32, chrono::NaiveDateTime, chrono::NaiveDateTime, User)>(conn)
        .map_err(|e| (Status::InternalServerError, format!("Erreur lors de la récupération des amis: {}", e)))?;

    let mut detailed_friends = Vec::new();

    // Pour chaque ami où l'utilisateur est user1
    for (id, user1_id, user2_id, created_at, updated_at, user2) in friends1 {
        let user1 = users_dsl::users
            .filter(users_dsl::id.eq(user1_id))
            .first::<User>(conn)
            .map_err(|_| (Status::InternalServerError, "Erreur lors de la récupération de l'utilisateur 1".to_string()))?;
        detailed_friends.push(DetailedFriend {
            id,
            user1,
            user2,
            created_at,
            updated_at,
        });
    }

    // Pour chaque ami où l'utilisateur est user2
    for (id, user1_id, user2_id, created_at, updated_at, user1) in friends2 {
        let user2 = users_dsl::users
            .filter(users_dsl::id.eq(user2_id))
            .first::<User>(conn)
            .map_err(|_| (Status::InternalServerError, "Erreur lors de la récupération de l'utilisateur 2".to_string()))?;
        detailed_friends.push(DetailedFriend {
            id,
            user1,
            user2,
            created_at,
            updated_at,
        });
    }

    Ok(detailed_friends)
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