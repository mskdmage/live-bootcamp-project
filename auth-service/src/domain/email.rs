// TODO: Evaluate optional libs to improve validation
// validator - provides common validation functions for emails, URLs, and more!
// fake - an easy to use library for generating fake data like name, number, address, lorem, dates, etc. This is useful for unit tests!
// rand -  provides random number generation capabilities. Used here to create seeded RNGs for deterministic test data generation with fake.
// quickcheck - provides a way to do property-based testing using randomly generated input. Property-based testing is a testing approach where you define properties (invariants, rules, or behaviors) that should always hold true for your code. The testing framework then automatically generates a wide range of inputs and checks if your code maintains those properties for all generated cases. This is useful for unit tests!
// quickcheck_macros - provides a convenient quickcheck! macro.

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(content: &str) -> Result<Self, EmailValidationError> {
        if content.len() > 128 || content.len() < 6 {
            return Err(EmailValidationError::InvalidLength);
        }

        let (_name, _suffix) = content
            .split_once('@')
            .ok_or(EmailValidationError::InvalidFormat)?;

        let (_subdomain, _tld) = _suffix
            .split_once('.')
            .ok_or(EmailValidationError::InvalidFormat)?;

        if _name.len() < 3 || _subdomain.len() < 3 || _tld.len() < 2 {
            return Err(EmailValidationError::InvalidFormat);
        }

        Ok(Self(content.to_owned()))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum EmailValidationError {
    InvalidLength,
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email_should_return_ok() {
        let valid_email = "test@test.com";
        let parsed_email = Email::parse(valid_email);

        assert_eq!(parsed_email, Ok(Email("test@test.com".to_owned())));
    }

    #[test]
    fn invalid_email_should_return_err() {
        let invalid_len_test_cases = ["t@t.c", ""];

        let invalid_format_test_cases = ["test_at_test.com", "test@testcom", "test@test."];

        for test_case in invalid_len_test_cases.iter() {
            let parsed_email = Email::parse(test_case);
            assert_eq!(
                parsed_email,
                Err(EmailValidationError::InvalidLength),
                "Failed for {}",
                test_case
            );
        }

        for test_case in invalid_format_test_cases.iter() {
            let parsed_email = Email::parse(test_case);
            assert_eq!(
                parsed_email,
                Err(EmailValidationError::InvalidFormat),
                "Failed for {}",
                test_case
            );
        }
    }
}
