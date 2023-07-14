mod constants;
mod password_cracker;
mod password_mode;
mod password_rules_attack;
mod processing_strategy;
mod struct_opt_options;
use std::error::Error;
use structopt::StructOpt;

use crate::password_cracker::PasswordCracker;
use crate::password_rules_attack::parse_rules;
use crate::struct_opt_options::Opt;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let password_cracker = PasswordCracker::new(&opt.hash)?;

    let rules = if let Some(rule_strings) = opt.rules {
        parse_rules(&rule_strings)
    } else {
        vec![]
    };

    let password_found = opt
        .mode
        .process_wordlist(&opt.wordlist, &password_cracker, &rules)?;

    if !password_found {
        println!("Password not found in the provided wordlist");
    }

    Ok(())
}
