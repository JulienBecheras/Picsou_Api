
use argon2::{self, PasswordHash, PasswordVerifier, Version};
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::password_hash::rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2::Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, Default::default());
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    if let Ok(parsed_hash) = parsed_hash {
        let argon2 = argon2::Argon2::default();
        argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}
