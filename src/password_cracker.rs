use crate::constants::SHA1_HEX_STRING_LENGTH;
use sha1::Digest;
use std::error::Error;

pub struct PasswordCracker {
    pub hash_to_crack: String,
}

impl PasswordCracker {
    pub fn new(hash_to_crack: &str) -> Result<Self, Box<dyn Error>> {
        if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
            return Err("sha1 hash is not valid".into());
        }
        Ok(Self {
            hash_to_crack: hash_to_crack.to_string(),
        })
    }

    pub fn check_password(&self, common_password: &str) -> bool {
        if self.hash_to_crack == hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Found password: {}", &common_password);
            true
        } else {
            false
        }
    }

    pub fn check_hash(&self, hash: &str) -> bool {
        let hashed_input = hex::encode(sha1::Sha1::digest(hash.as_bytes()));
        self.hash_to_crack == hashed_input
    }
        
}
