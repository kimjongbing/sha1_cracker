pub trait Rule: Sync {
    fn apply(&self, password: String) -> Vec<String>;
}

pub struct AppendDigitRule;
pub struct PrependDigitRule;
pub struct ReplaceLetterARule;
pub struct ReplaceLetterIRule;

impl Rule for AppendDigitRule {
    fn apply(&self, word: String) -> Vec<String> {
        (0..=9).map(|i| format!("{}{}", word, i)).collect()
    }
}

impl Rule for PrependDigitRule {
    fn apply(&self, word: String) -> Vec<String> {
        (0..=9).map(|i| format!("{}{}", i, word)).collect()
    }
}

impl Rule for ReplaceLetterARule {
    fn apply(&self, word: String) -> Vec<String> {
        vec![word.replace('a', "@")]
    }
}

impl Rule for ReplaceLetterIRule {
    fn apply(&self, word: String) -> Vec<String> {
        vec![word.replace('i', "1")]
    }
}

pub fn parse_rules(rule_strings: &[String]) -> Vec<Box<dyn Rule>> {
    rule_strings
        .iter()
        .flat_map(|rule| match rule.as_str() {
            "append" => vec![Box::new(AppendDigitRule) as Box<dyn Rule>],
            "prepend" => vec![Box::new(PrependDigitRule) as Box<dyn Rule>],
            "replace_a" => vec![Box::new(ReplaceLetterARule) as Box<dyn Rule>],
            "replace_i" => vec![Box::new(ReplaceLetterIRule) as Box<dyn Rule>],
            "all" => vec![
                Box::new(AppendDigitRule) as Box<dyn Rule>,
                Box::new(PrependDigitRule) as Box<dyn Rule>,
                Box::new(ReplaceLetterARule) as Box<dyn Rule>,
                Box::new(ReplaceLetterIRule) as Box<dyn Rule>,
            ],
            _ => panic!("Unknown rule: {}", rule),
        })
        .collect()
}
