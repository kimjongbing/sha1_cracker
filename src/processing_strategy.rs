use crate::password_cracker::PasswordCracker;
use crate::password_rules_attack::Rule;
use std::error::Error;

pub trait ProcessingStrategy {
    fn process_wordlist(
        &self,
        filename: &str,
        password_cracker: &PasswordCracker,
        rules: &[&dyn Rule],
    ) -> Result<bool, Box<dyn Error>>;

    fn apply_rules_and_check_password(
        &self,
        password: &str,
        password_cracker: &PasswordCracker,
        rules: &[&dyn Rule],
    ) -> bool {
        let mut variations = vec![password.to_string()];
        if !rules.is_empty() {
            for rule in rules {
                variations.extend(rule.apply(password.to_string()));
            }
        }
        for variant in variations {
            if password_cracker.check_password(&variant) {
                return true;
            }
        }
        false
    }
}