use crate::password_mode::PasswordMode;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "password cracker")]
pub struct Opt {
 
    pub hash: String,

   
    pub wordlist: String,

    /// Processing mode. Choices: mem, line, threads
    #[structopt(
        long,
        about = "mem - Load list into memory\nline - Read list line by line\nthreads - Use multiple threads"
    )]
    pub mode: PasswordMode,

    /// Optional rules for password modifications. Choices: append, prepend, replace_a, replace_i, all
    #[structopt(
        short = "r",
        long = "rule",
        about = "append - Append a digit\nprepend - Prepend a digit\nreplace_a - Replace letter 'a' with '@'\nreplace_i - Replace letter 'i' with '1'\nall - Apply all rules"
    )]
    pub rules: Option<Vec<String>>,
}
