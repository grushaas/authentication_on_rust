extern crate ring;
extern crate data_encoding;

use std::num::NonZeroU32;
use base64::{engine::general_purpose, Engine};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};

pub fn hash_password(pass: &String) -> Result<String, Box<dyn std::error::Error>> {
    const CREDENTIAL_LEN:usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt);

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        pass.as_bytes(),
        &mut pbkdf2_hash,
    );

    let salt_encoded = base64::engine::general_purpose::STANDARD.encode(&salt);
    let hash_encoded = base64::engine::general_purpose::STANDARD.encode(&pbkdf2_hash);

    Ok(format!("{}:{}", salt_encoded, hash_encoded))
}

pub fn verify_password(stored_hash: &str, password: String) -> Result<bool, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = stored_hash.split(':').collect();
    if parts.len() != 2 {
        return Ok(false); // Неверный формат
    }

    let salt_encoded = parts[0];
    let hash_encoded = parts[1];

    // Декодируем соль и хеш из Base64
    let salt = general_purpose::STANDARD.decode(salt_encoded)?;
    let stored_hash_bytes = general_purpose::STANDARD.decode(hash_encoded)?;

    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN; // Длина SHA-512 хеша
    let n_iter = NonZeroU32::new(100_000).unwrap();

    // Хешируем введенный пароль с той же солью
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    // Сравниваем хеши
    Ok(pbkdf2_hash == stored_hash_bytes.as_slice())
}