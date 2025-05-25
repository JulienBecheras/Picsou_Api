use rocket::{request::{self, FromRequest, Request}, outcome::Outcome};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use crate::utils::jwt::validate_jwt;// Assure-toi que validate_jwt est bien déclarée dans `session`

// Structure représentant les données du token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // ID de l'utilisateur
    pub exp: usize,   // Expiration du token
}

// Guard d'authentification
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub token: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Récupération du header Authorization
        let token = req.headers().get_one("Authorization");

        match token {
            Some(token) if token.starts_with("Bearer ") => {
                let token = &token[7..]; // Enlever "Bearer "

                // Utilisation de la fonction validate_jwt pour décoder le token
                match validate_jwt(token) {
                    Some(claims) => Outcome::Success(AuthenticatedUser {
                        user_id: claims.sub.parse().unwrap(),
                        token: token.to_string(),
                    }),
                    None => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
