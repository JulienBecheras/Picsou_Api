use rocket::{request::{self, FromRequest, Request}, outcome::Outcome};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use crate::utils::jwt::validate_jwt;

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
        let cookies = req.cookies();
        if let Some(cookie) = cookies.get("token") {
            let token = cookie.value();
            
            if let Some(claims) = validate_jwt(token) {
                return Outcome::Success(AuthenticatedUser {
                    user_id: claims.sub.parse().unwrap(),
                    token: token.to_string(),
                });
            } else {
                return Outcome::Error((Status::Unauthorized, ()));
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
