use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};

const SETUP_CODE_RANDOM_BYTES: usize = 8;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let mut salt_bytes = [0_u8; 16];

    getrandom::fill(&mut salt_bytes).map_err(|_| argon2::password_hash::Error::Crypto)?;

    let salt = SaltString::encode_b64(&salt_bytes)?;

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
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
    use argon2::password_hash::{PasswordHash, PasswordVerifier};

    #[test]
    fn password_hash_round_trip() {
        let password = "correct horse battery staple";

        let encoded = hash_password(password).expect("password should hash");

        let parsed = PasswordHash::new(&encoded).expect("generated hash should be valid");

        assert!(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok()
        );

        assert!(
            Argon2::default()
                .verify_password(b"definitely wrong", &parsed)
                .is_err()
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
}
