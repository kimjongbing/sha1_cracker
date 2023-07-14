use crate::constants::CHUNK_SIZE;
use crate::password_cracker::PasswordCracker;
use crate::password_rules_attack::Rule;
use crate::processing_strategy::ProcessingStrategy;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

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
        rules: &[Box<dyn Rule>],
    ) -> Result<bool, Box<dyn Error>> {
        Ok(chunk
            .par_iter()
            .any(|password| self.apply_rules_and_check_password(password, password_cracker, rules)))
    }
}

impl ProcessingStrategy for MemPasswordMode {
    fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
        rules: &[Box<dyn Rule>],
    ) -> Result<bool, Box<dyn Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let passwords: HashSet<&str> = contents.split('\n').collect();
        for password in passwords {
            if self.apply_rules_and_check_password(password, password_cracker, rules) {
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
        rules: &[Box<dyn Rule>],
    ) -> Result<bool, Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        while reader.read_line(&mut line)? > 0 {
            let password = line.trim();
            if self.apply_rules_and_check_password(password, password_cracker, rules) {
                return Ok(true);
            }
            line.clear();
        }

        Ok(false)
    }
}

impl ProcessingStrategy for ThreadsPasswordMode {
    fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
        rules: &[Box<dyn Rule>],
    ) -> Result<bool, Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut chunk = Vec::with_capacity(self.chunk_size);
        for line in reader.lines() {
            let line = line?;
            chunk.push(line);

            if chunk.len() >= self.chunk_size {
                if self.process_chunk(&chunk, password_cracker, rules)? {
                    return Ok(true);
                }
                chunk.clear();
            }
        }

        if self.process_chunk(&chunk, password_cracker, rules)? {
            return Ok(true);
        }

        Ok(false)
    }
}

impl FromStr for PasswordMode {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mem" => Ok(PasswordMode::Mem(Box::new(MemPasswordMode))),
            "line" => Ok(PasswordMode::Line(Box::new(LinePasswordMode))),
            "threads" => Ok(PasswordMode::Threads(Box::new(ThreadsPasswordMode::new(
                CHUNK_SIZE,
            )))),
            _ => Err("Invalid mode".into()),
        }
    }
}

impl fmt::Debug for PasswordMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PasswordMode::{:?}",
            match self {
                PasswordMode::Mem(_) => "Mem",
                PasswordMode::Line(_) => "Line",
                PasswordMode::Threads(_) => "Threads",
            }
        )
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
        rules: &[Box<dyn Rule>],
    ) -> Result<bool, Box<dyn Error>> {
        match self {
            PasswordMode::Mem(strategy) => {
                strategy.process_wordlist(filename, password_cracker, rules)
            }
            PasswordMode::Line(strategy) => {
                strategy.process_wordlist(filename, password_cracker, rules)
            }
            PasswordMode::Threads(strategy) => {
                strategy.process_wordlist(filename, password_cracker, rules)
            }
        }
    }
}
