use crate::constants::CHUNK_SIZE;
use crate::password_cracker::PasswordCracker;
use rayon::prelude::*;
use sha1::Digest;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub enum PasswordMode {
    Mem,
    Line,
    Threads,
}

impl PasswordMode {
    fn process_chunk(
        &self,
        chunk: &[String],
        password_cracker: &PasswordCracker,
    ) -> Result<bool, Box<dyn Error>> {
        Ok(chunk.par_iter().any(|line| {
            let common_password = line.trim();
            let hashed_password = hex::encode(sha1::Sha1::digest(common_password.as_bytes()));
            if password_cracker.hash_to_crack == hashed_password {
                println!("Found password: {}", &common_password);
                true
            } else {
                false
            }
        }))
    }

    pub fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
    ) -> Result<bool, Box<dyn Error>> {
        match self {
            PasswordMode::Mem => {
                let mut file = File::open(filename)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let passwords: Vec<&str> = contents.split('\n').collect();
                for password in passwords {
                    if password_cracker.check_password(password) {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            PasswordMode::Line => {
                let file = File::open(filename)?;
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line?;
                    if password_cracker.check_password(&line) {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            PasswordMode::Threads => {
                let file = File::open(filename)?;
                let reader = BufReader::new(file);

                let mut chunk = Vec::with_capacity(CHUNK_SIZE);
                for line in reader.lines() {
                    let line = line?;
                    chunk.push(line);

                    if chunk.len() >= CHUNK_SIZE {
                        if self.process_chunk(&chunk, password_cracker)? {
                            return Ok(true);
                        }
                        chunk.clear();
                    }
                }

                if self.process_chunk(&chunk, password_cracker)? {
                    return Ok(true);
                }

                Ok(false)
            }
        }
    }
}
