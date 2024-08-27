use anyhow::{anyhow, Result};
use password_hash::{PasswordHash, PasswordHasher, SaltString};
use pbkdf2::{Params, Pbkdf2};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

const SDK_PRIVATE_KEY: &[u8] = include_bytes!("../security/sdk_private_key.der");

#[must_use]
pub fn rsa_decrypt(cipher: &[u8]) -> Result<Vec<u8>> {
    let private_key: RsaPrivateKey = rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(SDK_PRIVATE_KEY)?;
    let payload = private_key.decrypt(Pkcs1v15Encrypt, cipher)?;

    Ok(payload.into())
}

#[must_use]
pub fn decrypt_string(cipher_b64: &str) -> Result<String> {
    let cipher = rbase64::decode(cipher_b64)?;
    let payload = rsa_decrypt(&cipher)?;

    let data = String::from_utf8(payload)?;
    Ok(data)
}

#[must_use]
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(rand::thread_rng());
    let hash = Pbkdf2
        .hash_password_customized(
            password.as_bytes(),
            None,
            None,
            Params {
                rounds: 10000,
                output_length: 32,
            },
            &salt,
        )
        .map_err(|_| anyhow!("Failed to generate password hash"))?;

    Ok(hash.serialize().to_string())
}

#[must_use]
pub fn verify_password(password: &str, hash_str: &str) -> Result<()> {
    let hash = PasswordHash::new(hash_str).map_err(|_| anyhow!("Failed to parse input hash"))?;
    hash.verify_password(&[&Pbkdf2], password)
        .map_err(|_| anyhow!("Password verification failed"))
}
