use rocket::http::Status;
use crate::repositories::friend_repository;
use crate::models::friend::{DetailedFriend, Friend, InsertableFriend};
use crate::models::friend_request::{DetailedFriendRequest, FriendRequest, InsertableFriendRequest};
use crate::models::user::User;
use crate::repositories::friend_repository::get_friend_request_by_id;

/// Vérifie si deux utilisateurs sont amis
pub fn are_they_friends(user1_id: i32, user2_id: i32) -> Result<bool, (Status, String)> {
    match friend_repository::get_friends_for_user(user1_id) {
        Ok(friends) => {
            if friends.iter().any(|friend| friend.user2.id == user2_id) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(e),
    }
}

/// Vérifie si deux utilisateurs ont une demande d'amis en cours
pub fn are_they_friends_request(from_user_id: i32, to_user_id: i32) -> Result<bool, (Status, String)> {
    //On veut savoir si l'un des deux utilisateurs a envoyé une demande d'amis à l'autre
    match friend_repository::get_friend_requests_for_user(from_user_id) {
        // Si l'une des requetes d'amis emises a l'intention de to_user_id à été émise par from_user_id, alors on est amis
        Ok(friends) => {
            if friends.iter().any(|friend| friend.from_user.id == to_user_id || friend.to_user.id == to_user_id) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(e),
    }

}


/// Crée une nouvelle demande d'amis. On vérifie d'abord si les utilisateurs sont déjà amis ou s'ils ont déjà une demande d'amis en cours.
pub fn create_friend_request(request: &InsertableFriendRequest) -> Result<FriendRequest, (Status, String)> {
    match are_they_friends(request.from_user_id, request.to_user_id) {
        Ok(areFriends) => {
            if areFriends {
                Err((Status::BadRequest, "Vous êtes déjà amis".to_string()))
            } else {
                match are_they_friends_request(request.from_user_id, request.to_user_id) {
                    Ok(areFriendRequests) => {
                        if areFriendRequests {
                            Err((Status::Conflict, "Vous avez déjà envoyé une demande d'amis".to_string()))
                        } else {
                            friend_repository::create_friend_request(request)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        }
        Err(e) => Err(e),
    }
}

/// Supprime une demande d'amis. On vérifie d'abord si la demande d'amis existe pour l'utilisateur.
pub fn delete_friend_request(user_id: i32, request_id: i32) -> Result<(), (Status, String)> {
    match get_friend_requests_for_user(user_id) {
        Ok(requests) => {
            if requests.iter().any(|request| request.id == request_id) {
                friend_repository::delete_friend_request(request_id)
            } else {
                Err((Status::NotFound, "Demande d'amis non trouvée pour cet utilisateur".to_string()))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn delete_friend(user_id: i32, friend_id: i32) -> Result<(), (Status, String)> {
    match get_friends_for_user(user_id) {
        Ok(requests) => {
            if requests.iter().any(|request| request.id == friend_id) {
                friend_repository::delete_friend(friend_id)
            } else {
                Err((Status::NotFound, "Amis non trouvée pour cet utilisateur".to_string()))
            }
        }
        Err(e) => Err(e),
    }
}

/// Accepte une demande d'amis. On vérifie d'abord si la demande d'amis existe et si l'utilisateur est bien le destinataire de la demande.
pub fn accept_friend_request(
    request_id: i32,
    user_id: i32,
) -> Result<Friend, (Status, String)> {
    match get_friend_request_by_id(request_id) {
        Ok(request) => {
            if request.to_user_id != user_id {
                return Err((Status::Forbidden, "Vous ne pouvez pas accepter cette demande d'amis".to_string()));
            }
            let new_friend = InsertableFriend {
                user1_id: request.from_user_id,
                user2_id: request.to_user_id,
            };
            match friend_repository::create_friend(&new_friend) {
                Ok(friend) => {
                    // Ensuite, on supprime la demande d'amis
                    match friend_repository::delete_friend_request(request_id) {
                        Ok(_) => Ok(friend),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}


// Récupérer toutes les demandes d'amis pour un utilisateur
pub fn get_friend_requests_for_user(user_id: i32) -> Result<Vec<DetailedFriendRequest>, (Status, String)> {
    friend_repository::get_friend_requests_for_user(user_id)
}

// Récupérer tous les amis d'un utilisateur
pub fn get_friends_for_user(user_id: i32) -> Result<Vec<DetailedFriend>, (Status, String)> {
    friend_repository::get_friends_for_user(user_id)
}