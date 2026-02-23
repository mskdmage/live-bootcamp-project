// TODO: Evaluate optional libs to improve validation
// validator - provides common validation functions for emails, URLs, and more! 
// fake - an easy to use library for generating fake data like name, number, address, lorem, dates, etc. This is useful for unit tests!
// rand -  provides random number generation capabilities. Used here to create seeded RNGs for deterministic test data generation with fake.  
// quickcheck - provides a way to do property-based testing using randomly generated input. Property-based testing is a testing approach where you define properties (invariants, rules, or behaviors) that should always hold true for your code. The testing framework then automatically generates a wide range of inputs and checks if your code maintains those properties for all generated cases. This is useful for unit tests! 
// quickcheck_macros - provides a convenient quickcheck! macro.

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(content: &str) -> Result<Self, PasswordValidationError> {
        
        if content.len() < 8 {
            return Err(PasswordValidationError::InvalidLength);
        }

        Ok(Self(content.to_owned()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum PasswordValidationError {
    InvalidLength,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_password_should_return_ok() {

        let valid_password = "StrongPass#123$";
        let parsed_password = Password::parse(valid_password);

        assert_eq!(
            parsed_password,
            Ok(Password("StrongPass#123$".to_owned()))
        );
    }

    #[test]
    fn invalid_password_should_return_err() {

        let invalid_len_test_cases = [
            "pass1",
        ];

        for test_case in invalid_len_test_cases.iter() {
            let parsed_password = Password::parse(test_case);
            assert_eq!(
                parsed_password,
                Err(PasswordValidationError::InvalidLength),
                "Failed for {}",
                test_case
            );
        }

    }
}