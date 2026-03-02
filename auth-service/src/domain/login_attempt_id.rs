use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        let id = Uuid::parse_str(&id);
        id.map(|id| Self(id.to_string())).map_err(|e| e.to_string())
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_login_attempt_id_should_return_ok() {
        let uuid = Uuid::new_v4().to_string();
        let parsed = LoginAttemptId::parse(uuid.clone());

        assert_eq!(parsed, Ok(LoginAttemptId(uuid)));
    }

    #[test]
    fn invalid_login_attempt_id_should_return_err() {
        let invalid_ids = ["not-a-uuid", "", "1234"];

        for case in invalid_ids {
            let parsed = LoginAttemptId::parse(case.to_owned());
            assert!(parsed.is_err(), "Expected error for {}", case);
        }
    }

    #[test]
    fn default_generates_valid_uuid_string() {
        let id = LoginAttemptId::default();
        let parsed = Uuid::parse_str(id.as_ref());
        assert!(parsed.is_ok());
    }
}
