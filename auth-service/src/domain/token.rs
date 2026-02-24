// TODO: Evaluate optional libs to improve validation
// validator - provides common validation functions for emails, URLs, and more! 
// fake - an easy to use library for generating fake data like name, number, address, lorem, dates, etc. This is useful for unit tests!
// rand -  provides random number generation capabilities. Used here to create seeded RNGs for deterministic test data generation with fake.  
// quickcheck - provides a way to do property-based testing using randomly generated input. Property-based testing is a testing approach where you define properties (invariants, rules, or behaviors) that should always hold true for your code. The testing framework then automatically generates a wide range of inputs and checks if your code maintains those properties for all generated cases. This is useful for unit tests! 
// quickcheck_macros - provides a convenient quickcheck! macro.

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token(String);

impl Token {
    pub fn parse(content: &str) -> Result<Self, TokenValidationError> {
        
        if content.len() < 10 {
            return Err(TokenValidationError::InvalidLength);
        }

        // JWT tokens have format: header.payload.signature (3 parts separated by dots)
        let parts: Vec<&str> = content.split('.').collect();
        
        if parts.len() != 3 {
            return Err(TokenValidationError::InvalidFormat);
        }

        // Each part should have content
        if parts[0].is_empty() || parts[1].is_empty() || parts[2].is_empty() {
            return Err(TokenValidationError::InvalidFormat);
        }

        Ok(Self(content.to_owned()))
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenValidationError {
    InvalidLength,
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_token_should_return_ok() {
        // A minimal valid JWT format (header.payload.signature)
        let valid_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0QGV4YW1wbGUuY29tIiwiZXhwIjoxNjAwMDAwMDAwfQ.signature";
        let parsed_token = Token::parse(valid_token);

        assert_eq!(
            parsed_token,
            Ok(Token(valid_token.to_owned()))
        );
    }

    #[test]
    fn invalid_token_should_return_err() {
        let invalid_len_test_cases = [
            "a.b.c",      // Too short
            "short",      // Too short
            "",           // Empty
        ];

        let invalid_format_test_cases = [
            "header.payload",           // Missing signature (only 2 parts)
            "header.payload.signature.extra", // Too many parts (4 parts)
            "header..signature",         // Empty middle part
            ".payload.signature",         // Empty first part
            "header.payload.",           // Empty last part
            "no-dots-here",              // No dots at all
        ];

        for test_case in invalid_len_test_cases.iter() {
            let parsed_token = Token::parse(test_case);
            assert_eq!(
                parsed_token,
                Err(TokenValidationError::InvalidLength),
                "Failed for {}",
                test_case
            );
        }

        for test_case in invalid_format_test_cases.iter() {
            let parsed_token = Token::parse(test_case);
            assert_eq!(
                parsed_token,
                Err(TokenValidationError::InvalidFormat),
                "Failed for {}",
                test_case
            );
        }
    }
}
