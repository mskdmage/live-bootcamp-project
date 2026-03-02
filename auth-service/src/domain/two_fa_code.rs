use rand::{self, random_range};

#[derive(Debug, Clone, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid code".to_owned());
        }
        Ok(Self(code))
    } 
}

impl Default for TwoFACode {

    fn default() -> Self {
        let rgen = random_range(0..=999999);
        Self(format!("{:06}", rgen))
    }
}