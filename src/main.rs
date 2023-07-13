mod constants;
mod password_cracker;
mod password_mode;
use crate::constants::CHUNK_SIZE;
use crate::constants::SHA1_HEX_STRING_LENGTH;
use crate::password_cracker::PasswordCracker;
use crate::password_mode::LinePasswordMode;
use crate::password_mode::MemPasswordMode;
use crate::password_mode::PasswordMode;
use crate::password_mode::ThreadsPasswordMode;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: ");
        println!("sha_1cracker: <wordlist.txt> <sha1_hash> <mode>");
        println!("Mode:");
        println!(" mem - Load list into memory");
        println!(" line - Read list line by line");
        println!(" threads - Use multiple threads");

        return Ok(());
    }

    let hash_to_crack: &str = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let password_cracker = PasswordCracker::new(hash_to_crack)?;

    let mode = match args[3].as_str() {
        "mem" => PasswordMode::Mem(MemPasswordMode),
        "line" => PasswordMode::Line(LinePasswordMode),
        "threads" => PasswordMode::Threads(ThreadsPasswordMode::new(CHUNK_SIZE)),
        _ => return Err("Invalid mode".into()),
    };

    let password_found = mode.process_wordlist(&args[1], &password_cracker)?;

    if !password_found {
        println!("Password not found in wordlist");
    }
    Ok(())
}
