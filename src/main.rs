mod vault;

use vault::SecretVault;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    print!("Enter passphrase: ");
    io::stdout().flush().unwrap();
    let mut passphrase = String::new();
    io::stdin().read_line(&mut passphrase).unwrap();
    let encryption_key = SecretVault::derive_key(&passphrase.trim());

    let mut secret_vault = SecretVault::new();
    loop {
        let mut input = String::new();
        print!("vault> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["add", key, secret] => {
                secret_vault.add_secret(&encryption_key, key.to_string(), secret.to_string());
                println!("Secret added!");
            },
            ["get", key] => {
                if let Some(secret) = secret_vault.get_secret(&encryption_key, key) {
                    println!("Secret: {}", secret);
                } else {
                    println!("No secret found for key: {}", key);
                }
            },
            ["remove", key] => {
                if secret_vault.remove_secret(&encryption_key, key).is_some() {
                    println!("Secret removed!");
                } else {
                    println!("No secret found for key: {}", key);
                }
            },
            ["save", file_path] => {
                if secret_vault.save_to_file(Path::new(file_path)).is_ok() {
                    println!("Vault saved to {}", file_path);
                } else {
                    println!("Error saving vault to {}", file_path);
                }
            },
            ["load", file_path] => {
                match SecretVault::from_file(Path::new(file_path)) {
                    Ok(vault) => {
                        secret_vault = vault;
                        println!("Vault loaded from {}", file_path);
                    },
                    Err(_) => println!("Error loading vault from {}", file_path),
                }
            },
            ["exit"] => {
                break;
            },
            _ => {
                println!("Invalid command!");
            }
        }
    }
}
