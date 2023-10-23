extern crate crypto;
use crypto::pbkdf2::pbkdf2;
use crypto::sha2::Sha256;
use aes_gcm::KeyInit;
use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead};
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crypto::hmac::Hmac;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SecretVault {
    secrets: HashMap<String, Vec<u8>>,
}

impl SecretVault {
    pub fn new() -> Self {
        SecretVault {
            secrets: HashMap::new(),
        }
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read(path)?;
        let vault: SecretVault = bincode::deserialize(&data)?;
        Ok(vault)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let data = bincode::serialize(&self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn derive_key(passphrase: &str) -> [u8; 32] {
        let salt: [u8; 16] = rand::thread_rng().gen();

        // Using pbkdf2 with Hmac and Sha256 to derive the key
        let mut key = [0u8; 32];
        let mut mac = Hmac::new(Sha256::new(), passphrase.as_bytes());
        pbkdf2(&mut mac, &salt, 10000, &mut key); // 10000 iterations
        key
    }


    // The following functions now take an encryption_key parameter:
    pub fn add_secret(&mut self, encryption_key: &[u8; 32], key: String, secret: String) {
        let cipher = Aes256Gcm::new_from_slice(encryption_key).unwrap();
        let nonce: [u8; 12] = rand::thread_rng().gen();
        let encrypted_secret = cipher.encrypt(&nonce.into(), secret.as_bytes()).unwrap();
        let mut combined = nonce.to_vec();
        combined.extend(encrypted_secret);
        self.secrets.insert(key, combined);
    }

    pub fn get_secret(&self, encryption_key: &[u8; 32], key: &str) -> Option<String> {
        if let Some(data) = self.secrets.get(key) {
            let cipher = Aes256Gcm::new_from_slice(encryption_key).unwrap();
            let (nonce, encrypted_secret) = data.split_at(12);
            if let Ok(decrypted_secret) = cipher.decrypt(nonce.into(), encrypted_secret) {
                return Some(String::from_utf8(decrypted_secret).unwrap());
            }
        }
        None
    }

    pub fn remove_secret(&mut self, encryption_key: &[u8; 32], key: &str) -> Option<String> {
        if let Some(data) = self.secrets.remove(key) {
            let cipher = Aes256Gcm::new_from_slice(encryption_key).unwrap();
            let (nonce, encrypted_secret) = data.split_at(12);
            if let Ok(decrypted_secret) = cipher.decrypt(nonce.into(), encrypted_secret) {
                return Some(String::from_utf8(decrypted_secret).unwrap());
            }
        }
        None
    }
}
