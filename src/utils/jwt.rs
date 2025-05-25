
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

const SECRET_KEY: &[u8] = b"super_secret_key"; // À stocker dans un fichier .env

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // L'identifiant de l'utilisateur
    pub exp: usize,   // Expiration du token
}

// Génère un token JWT
pub fn create_jwt(user_id: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(365))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    ).expect("Token creation failed")
}

// Vérifie le token
pub fn validate_jwt(token: &str) -> Option<Claims> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    );

    decoded.ok().map(|data| data.claims)
}