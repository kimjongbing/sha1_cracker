use crate::password_cracker::PasswordCracker;
use rayon::prelude::*;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub trait ProcessingStrategy {
    fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
    ) -> Result<bool, Box<dyn Error>>;
}

pub trait MemProcessingStrategy: ProcessingStrategy {}
pub trait LineProcessingStrategy: ProcessingStrategy {}
pub trait ThreadsProcessingStrategy: ProcessingStrategy {}

pub struct MemPasswordMode;
pub struct LinePasswordMode;
pub struct ThreadsPasswordMode {
    chunk_size: usize,
}

impl MemProcessingStrategy for MemPasswordMode {}
impl LineProcessingStrategy for LinePasswordMode {}
impl ThreadsProcessingStrategy for ThreadsPasswordMode {}

impl ThreadsPasswordMode {
    pub fn new(chunk_size: usize) -> Self {
        ThreadsPasswordMode { chunk_size }
    }

    fn process_chunk(
        &self,
        chunk: &[String],
        password_cracker: &PasswordCracker,
    ) -> Result<bool, Box<dyn Error>> {
        Ok(chunk.par_iter().any(|line| {
            let common_password = line.trim();
            if password_cracker.check_hash(common_password) {
                println!("Found password: {}", &common_password);
                true
            } else {
                false
            }
        }))
    }
}

impl ProcessingStrategy for MemPasswordMode {
    fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
    ) -> Result<bool, Box<dyn Error>> {
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
}

impl ProcessingStrategy for LinePasswordMode {
    fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
    ) -> Result<bool, Box<dyn Error>> {
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
}

impl ProcessingStrategy for ThreadsPasswordMode {
    fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
    ) -> Result<bool, Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut chunk = Vec::with_capacity(self.chunk_size);
        for line in reader.lines() {
            let line = line?;
            chunk.push(line);

            if chunk.len() >= self.chunk_size {
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

pub enum PasswordMode {
    Mem(Box<dyn MemProcessingStrategy>),
    Line(Box<dyn LineProcessingStrategy>),
    Threads(Box<dyn ThreadsProcessingStrategy>),
}

impl PasswordMode {
    pub fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
    ) -> Result<bool, Box<dyn Error>> {
        match self {
            PasswordMode::Mem(strategy) => strategy.process_wordlist(filename, password_cracker),
            PasswordMode::Line(strategy) => strategy.process_wordlist(filename, password_cracker),
            PasswordMode::Threads(strategy) => {
                strategy.process_wordlist(filename, password_cracker)
            }
        }
    }
}
