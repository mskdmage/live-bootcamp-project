use std::collections::{hash_map::Entry, HashMap};

use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        match self.codes.entry(email.clone()) {
            Entry::Vacant(e) => {
                e.insert((login_attempt_id, code));
            }
            Entry::Occupied(mut e) => {
                e.insert((login_attempt_id, code));
            }
        }

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        if !self.codes.contains_key(email) {
            return Err(TwoFACodeStoreError::LoginAttemptIdNotFound);
        }

        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(e) => Ok(e.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Email, LoginAttemptId, TwoFACode};
    use uuid::Uuid;

    fn attempt_id() -> LoginAttemptId {
        LoginAttemptId::parse(Uuid::new_v4().to_string()).unwrap()
    }

    #[tokio::test]
    async fn add_code_inserts_when_vacant() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("user@test.com").unwrap();
        let login_attempt_id = attempt_id();
        let two_fa_code = TwoFACode::parse("000123".to_owned()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
            .await
            .unwrap();

        let got = store.get_code(&email).await.unwrap();
        assert_eq!(got, (login_attempt_id, two_fa_code));
    }

    #[tokio::test]
    async fn add_code_overwrites_when_occupied() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("user@test.com").unwrap();

        store
            .add_code(
                email.clone(),
                attempt_id(),
                TwoFACode::parse("000123".to_owned()).unwrap(),
            )
            .await
            .unwrap();

        let login_attempt_id2 = attempt_id();
        let two_fa_code2 = TwoFACode::parse("000124".to_owned()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id2.clone(), two_fa_code2.clone())
            .await
            .unwrap();

        let got = store.get_code(&email).await.unwrap();
        assert_eq!(got, (login_attempt_id2, two_fa_code2));
    }

    #[tokio::test]
    async fn get_code_returns_error_when_missing() {
        let store = HashmapTwoFACodeStore::default();

        let email = Email::parse("user@test.com").unwrap();
        let err = store.get_code(&email).await.unwrap_err();

        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn remove_code_removes_existing() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("user@test.com").unwrap();
        store
            .add_code(
                email.clone(),
                attempt_id(),
                TwoFACode::parse("000124".to_owned()).unwrap(),
            )
            .await
            .unwrap();

        store.remove_code(&email).await.unwrap();

        let err = store.get_code(&email).await.unwrap_err();
        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn remove_code_returns_error_when_missing() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("user@test.com").unwrap();
        let err = store.remove_code(&email).await.unwrap_err();

        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn codes_are_isolated_per_email() {
        let mut store = HashmapTwoFACodeStore::default();

        let email1 = Email::parse("aaa@test.com").unwrap();
        let email2 = Email::parse("bbb@test.com").unwrap();

        store
            .add_code(
                email1.clone(),
                attempt_id(),
                TwoFACode::parse("000123".to_owned()).unwrap(),
            )
            .await
            .unwrap();

        store
            .add_code(
                email2.clone(),
                attempt_id(),
                TwoFACode::parse("000124".to_owned()).unwrap(),
            )
            .await
            .unwrap();

        let got1 = store.get_code(&email1).await.unwrap();
        let got2 = store.get_code(&email2).await.unwrap();

        assert_eq!(got1.1, TwoFACode::parse("000123".to_owned()).unwrap());
        assert_eq!(got2.1, TwoFACode::parse("000124".to_owned()).unwrap());
        assert_ne!(got1, got2);
    }
}
