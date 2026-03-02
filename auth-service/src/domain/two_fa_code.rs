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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_two_fa_code_should_return_ok() {
        let code = "123456".to_owned();
        let parsed = TwoFACode::parse(code.clone());

        assert_eq!(parsed, Ok(TwoFACode(code)));
    }

    #[test]
    fn invalid_two_fa_code_should_return_err() {
        let invalid_cases = [
            "12345",   // too short
            "1234567", // too long
            "12a456",  // non-digit
            "",        // empty
        ];

        for case in invalid_cases {
            let parsed = TwoFACode::parse(case.to_owned());
            assert_eq!(
                parsed,
                Err("Invalid code".to_owned()),
                "Failed for {}",
                case
            );
        }
    }

    #[test]
    fn default_generates_six_digit_code() {
        let code = TwoFACode::default();
        assert_eq!(code.0.len(), 6);
        assert!(code.0.chars().all(|c| c.is_ascii_digit()));
    }
}
