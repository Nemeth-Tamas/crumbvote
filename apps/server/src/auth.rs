use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use sha2::{Digest, Sha256};

const SETUP_CODE_RANDOM_BYTES: usize = 8;
const SESSION_TOKEN_RANDOM_BYTES: usize = 32;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let mut salt_bytes = [0_u8; 16];

    getrandom::fill(&mut salt_bytes).map_err(|_| argon2::password_hash::Error::Crypto)?;

    let salt = SaltString::encode_b64(&salt_bytes)?;

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(password_hash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub fn generate_session_token() -> Result<String, getrandom::Error> {
    let mut bytes = [0_u8; SESSION_TOKEN_RANDOM_BYTES];

    getrandom::fill(&mut bytes)?;

    Ok(hex::encode(bytes))
}

pub fn hash_session_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

pub fn generate_setup_code() -> Result<String, getrandom::Error> {
    let mut bytes = [0_u8; SETUP_CODE_RANDOM_BYTES];

    getrandom::fill(&mut bytes)?;

    let encoded = hex::encode_upper(bytes);

    Ok(format!(
        "{}-{}-{}-{}",
        &encoded[0..4],
        &encoded[4..8],
        &encoded[8..12],
        &encoded[12..16],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_round_trip() {
        let password = "correct horse battery staple";

        let encoded = hash_password(password).expect("password should hash");

        assert!(verify_password(password, &encoded).expect("generated hash should verify"));

        assert!(
            !verify_password("definitely wrong", &encoded)
                .expect("generated hash should be readable")
        );
    }

    #[test]
    fn setup_code_has_expected_shape() {
        let code = generate_setup_code().expect("setup code should generate");

        assert_eq!(code.len(), 19);
        assert_eq!(code.chars().filter(|c| *c == '-').count(), 3);

        for part in code.split('-') {
            assert_eq!(part.len(), 4);
            assert!(part.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn session_token_has_expected_shape() {
        let token = generate_session_token().expect("session token should generate");

        assert_eq!(token.len(), 64);

        assert!(
            token
                .chars()
                .all(|character| { character.is_ascii_hexdigit() })
        );
    }

    #[test]
    fn session_token_hash_is_sha256() {
        assert_eq!(
            hash_session_token("test-session-token"),
            "7a16f44e82f892c5db994ff1fe2c468656ad31af77ebe04b1d02be3bf8d4cc8e"
        );
    }
}
