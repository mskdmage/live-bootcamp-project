use std::collections::{HashMap, hash_map::Entry};

use crate::domain::{
    LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError, Email,
};

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
            },
            Entry::Occupied(mut e) => {
                e.insert((login_attempt_id, code));
            },
        }

        Ok(())
    }

    async fn remove_code(
        &mut self,
        email: &Email,
    ) -> Result<(), TwoFACodeStoreError> {

        if self.codes.get(email).is_none() {
            return Err(TwoFACodeStoreError::LoginAttemptIdNotFound);
        }

        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(e) => Ok(e.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

// TODO: implement TwoFACodeStore for HashmapTwoFACodeStore

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use crate::domain::{Email, LoginAttemptId, TwoFACode};

    fn attempt_id() -> LoginAttemptId {
        LoginAttemptId::parse(Uuid::new_v4().to_string()).unwrap()
    }

    #[tokio::test]
    async fn add_code_inserts_when_vacant() {
        let mut store = HashmapTwoFACodeStore::default();

        let e = Email::parse("user@test.com").unwrap();
        let a = attempt_id();
        let c = TwoFACode::parse("000123".to_owned()).unwrap();

        store.add_code(e.clone(), a.clone(), c.clone()).await.unwrap();

        let got = store.get_code(&e).await.unwrap();
        assert_eq!(got, (a, c));
    }

    #[tokio::test]
    async fn add_code_overwrites_when_occupied() {
        let mut store = HashmapTwoFACodeStore::default();

        let e = Email::parse("user@test.com").unwrap();

        store
            .add_code(e.clone(), attempt_id(), TwoFACode::parse("000123".to_owned()).unwrap())
            .await
            .unwrap();

        let a2 = attempt_id();
        let c2 = TwoFACode::parse("000124".to_owned()).unwrap();

        store.add_code(e.clone(), a2.clone(), c2.clone()).await.unwrap();

        let got = store.get_code(&e).await.unwrap();
        assert_eq!(got, (a2, c2));
    }

    #[tokio::test]
    async fn get_code_returns_error_when_missing() {
        let store = HashmapTwoFACodeStore::default();

        let e = Email::parse("user@test.com").unwrap();
        let err = store.get_code(&e).await.unwrap_err();

        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn remove_code_removes_existing() {
        let mut store = HashmapTwoFACodeStore::default();

        let e = Email::parse("user@test.com").unwrap();
        store
            .add_code(e.clone(), attempt_id(), TwoFACode::parse("000124".to_owned()).unwrap())
            .await
            .unwrap();

        store.remove_code(&e).await.unwrap();

        let err = store.get_code(&e).await.unwrap_err();
        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn remove_code_returns_error_when_missing() {
        let mut store = HashmapTwoFACodeStore::default();

        let e = Email::parse("user@test.com").unwrap();
        let err = store.remove_code(&e).await.unwrap_err();

        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn codes_are_isolated_per_email() {
        let mut store = HashmapTwoFACodeStore::default();

        let e1 = Email::parse("aaa@test.com").unwrap();
        let e2 = Email::parse("bbb@test.com").unwrap();

        store
            .add_code(e1.clone(), attempt_id(), TwoFACode::parse("000123".to_owned()).unwrap())
            .await
            .unwrap();

        store
            .add_code(e2.clone(), attempt_id(), TwoFACode::parse("000124".to_owned()).unwrap())
            .await
            .unwrap();

        let got1 = store.get_code(&e1).await.unwrap();
        let got2 = store.get_code(&e2).await.unwrap();

        assert_eq!(got1.1, TwoFACode::parse("000123".to_owned()).unwrap());
        assert_eq!(got2.1, TwoFACode::parse("000124".to_owned()).unwrap());
        assert_ne!(got1, got2);
    }
}