use std::{io::Write, path::Path};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use rsa::{
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    pkcs1v15::{Signature, SigningKey, VerifyingKey},
    signature::{SignatureEncoding, SignerMut, Verifier},
};
use sha2::Sha256;
use tokio::io::AsyncReadExt;

pub fn hash_password(password: &str) -> String {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
    password_hash
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let password = password.as_bytes();
    let Ok(parsed_hash) = PasswordHash::new(password_hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok()
}

pub fn sign_rsa(priv_key: &RsaPrivateKey, plaintext: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut signing_key = SigningKey::<Sha256>::new(priv_key.to_owned());
    let signature = signing_key.sign(plaintext);
    let signature = signature.to_bytes().to_vec();

    Ok(signature)
}

pub fn verify_rsa(
    pub_key: &RsaPublicKey,
    msg: &str,
    signature: &[u8],
) -> Result<(), anyhow::Error> {
    let verifying_key = VerifyingKey::<Sha256>::new(pub_key.to_owned());
    let signature = Signature::try_from(signature)?;
    verifying_key.verify(msg.as_bytes(), &signature)?;

    Ok(())
}

pub fn decrypt_rsa(priv_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let plaintext = priv_key.decrypt(Pkcs1v15Encrypt, ciphertext)?;

    Ok(plaintext)
}

pub fn decrypt_rsa_to_utf8(
    priv_key: &RsaPrivateKey,
    ciphertext: &[u8],
) -> Result<String, anyhow::Error> {
    let plaintext = decrypt_rsa(priv_key, ciphertext)?;
    let plaintext = String::from_utf8(plaintext)?;

    Ok(plaintext)
}

pub fn encrypt_rsa(pub_key: &RsaPublicKey, ciphertext: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut rng = rand::thread_rng();
    let ciphertext = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, ciphertext)?;

    Ok(ciphertext)
}

pub fn encrypt_rsa_to_utf8(
    pub_key: &RsaPublicKey,
    ciphertext: &[u8],
) -> Result<String, anyhow::Error> {
    let ciphertext = encrypt_rsa(pub_key, ciphertext)?;
    let ciphertext = String::from_utf8(ciphertext)?;

    Ok(ciphertext)
}

pub fn decode_base64_to_utf8(base64_str: &str) -> Result<String, anyhow::Error> {
    let decoded = decode_base64(base64_str)?;
    let decoded = String::from_utf8(decoded)?;

    Ok(decoded)
}

pub fn decode_base64(base64_str: &str) -> Result<Vec<u8>, anyhow::Error> {
    let decoded = BASE64_STANDARD.decode(base64_str.as_bytes())?;

    Ok(decoded)
}

pub fn encode_base64(input: &[u8]) -> String {
    BASE64_STANDARD.encode(input)
}

pub fn decode_url(input: &str) -> Result<String, anyhow::Error> {
    Ok(urlencoding::decode(input)?.to_string())
}

pub fn encode_url(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

pub fn hash_blake3(input: &[u8]) -> String {
    blake3::hash(input).to_string()
}

pub fn hash_md5(input: &[u8]) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

pub async fn hash_file_md5(path: &Path) -> String {
    let mut context = md5::Context::new();
    let mut file = tokio::fs::File::open(path).await.unwrap();
    let mut buffer = vec![0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer).await.unwrap();

        if bytes_read == 0 {
            break;
        }

        context.write(&buffer[..bytes_read]).unwrap();
    }

    let digest = context.finalize();

    format!("{:x}", digest)
}
