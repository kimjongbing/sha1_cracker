use rayon::prelude::*;
use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

const SHA1_HEX_STRING_LENGTH: usize = 40;
const CHUNK_SIZE: usize = 100_000;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: ");
        println!("sha_1cracker: <wordlist.txt> <sha1_hash>");
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

    macro_rules! check_password {
        ($common_password:expr, $hash_to_crack:expr) => {{
            if $hash_to_crack == hex::encode(sha1::Sha1::digest($common_password.as_bytes())) {
                println!("Found password: {}", &$common_password);
                true
            } else {
                false
            }
        }};
    }

    macro_rules! process_wordlist {
        ($func:ident, $args1:expr, $args2:expr) => {
            match $func($args1, $args2)? {
                true => return Ok(()),
                _ => (),
            }
        };
    }

    let mode = &args[3];
    match mode.as_str() {
        "mem" => process_wordlist!(load_into_memory, &args[1], hash_to_crack),
        "line" => process_wordlist!(read_line_by_line, &args[1], hash_to_crack),
        "threads" => process_wordlist!(use_multiple_threads, &args[1], hash_to_crack),
        _ => return Err("Invalid mode".into()),
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn load_into_memory(filename: &str, hash_to_crack: &str) -> Result<bool, Box<dyn Error>> {
        let wordlist_file = File::open(filename)?;
        let mut reader = BufReader::new(wordlist_file);
        let mut wordlist = String::new();
        reader.read_to_string(&mut wordlist)?;

        let wordlist: Vec<&str> = wordlist.lines().collect();

        for &common_password in wordlist.iter() {
            if check_password!(common_password, hash_to_crack) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn read_line_by_line(filename: &str, hash_to_crack: &str) -> Result<bool, Box<dyn Error>> {
        for common_password in read_lines(filename)?.flatten() {
            if check_password!(common_password, hash_to_crack) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn process_chunk(chunk: &[String], hash_to_crack: &str) -> Result<bool, Box<dyn Error>> {
        Ok(chunk.par_iter().any(|line| {
            let common_password = line.trim();
            let hashed_password = hex::encode(sha1::Sha1::digest(common_password.as_bytes()));
            if hash_to_crack == hashed_password {
                println!("Found password: {}", &common_password);
                true
            } else {
                false
            }
        }))
    }

    fn use_multiple_threads(filename: &str, hash_to_crack: &str) -> Result<bool, Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut chunk = Vec::with_capacity(CHUNK_SIZE);
        for line in reader.lines() {
            let line = line?;
            chunk.push(line);

            if chunk.len() >= CHUNK_SIZE {
                if process_chunk(&chunk, hash_to_crack)? {
                    return Ok(true);
                }
                chunk.clear();
            }
        }

        if process_chunk(&chunk, hash_to_crack)? {
            return Ok(true);
        }

        Ok(false)
    }

    println!("Password not found in wordlist");
    Ok(())
}