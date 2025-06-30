use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome;
use serde::{Deserialize, Serialize};
use crate::utils::jwt::validate_jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        if let Some(cookie) = cookies.get("token") {
            let token = cookie.value();

            if let Some(claims) = validate_jwt(token) {
                if let Ok(user_id) = claims.sub.parse::<i32>() {
                    return Outcome::Success(AuthenticatedUser {
                        user_id,
                        token: token.to_string(),
                    });
                } else {
                    eprintln!("❌ Token JWT: `sub` n'est pas un i32 valide");
                }
            } else {
                eprintln!("❌ Token JWT invalide ou expiré");
            }
        } else {
            eprintln!("❌ Cookie `token` non trouvé");
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

