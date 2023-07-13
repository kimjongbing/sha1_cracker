mod constants;
mod password_cracker;
mod password_mode;
use crate::constants::SHA1_HEX_STRING_LENGTH;
use crate::password_cracker::PasswordCracker;
use crate::password_mode::PasswordMode;
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

    let mode = &args[3];
    let password_found = match mode.as_str() {
        "mem" => PasswordMode::Mem.process_wordlist(&args[1], &password_cracker)?,
        "line" => PasswordMode::Line.process_wordlist(&args[1], &password_cracker)?,
        "threads" => PasswordMode::Threads.process_wordlist(&args[1], &password_cracker)?,
        _ => return Err("Invalid mode".into()),
    };

    if !password_found {
        println!("Password not found in wordlist");
    }
    Ok(())
}
