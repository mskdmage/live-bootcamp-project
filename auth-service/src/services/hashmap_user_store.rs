use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email.clone()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned() // Cloning to comply with function signature
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;

        if password != &user.password {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let new_user = User::new(
            Email::parse("test@test.com").unwrap(),
            Password::parse("password123").unwrap(),
            true,
        );
        let same_user = new_user.clone();

        assert_eq!(store.add_user(new_user.clone()).await, Ok(()));
        assert_eq!(
            store.add_user(same_user).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let new_user = User::new(
            Email::parse("test@test.com").unwrap(),
            Password::parse("password123").unwrap(),
            true,
        );
        let email = new_user.email.clone();

        store.add_user(new_user.clone()).await.unwrap();

        let fetched = store.get_user(&email).await;
        assert_eq!(fetched, Ok(new_user));

        assert_eq!(
            store
                .get_user(&Email::parse("notfound@test.com").unwrap())
                .await,
            Err(UserStoreError::UserNotFound)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let new_user = User::new(
            Email::parse("test@test.com").unwrap(),
            Password::parse("password123").unwrap(),
            true,
        );

        store.add_user(new_user).await.unwrap();

        assert_eq!(
            store
                .validate_user(
                    &Email::parse("wrong@test.com").unwrap(),
                    &Password::parse("password123").unwrap()
                )
                .await,
            Err(UserStoreError::UserNotFound)
        );

        assert_eq!(
            store
                .validate_user(
                    &Email::parse("test@test.com").unwrap(),
                    &Password::parse("wrongpassword").unwrap()
                )
                .await,
            Err(UserStoreError::InvalidCredentials)
        );

        assert_eq!(
            store
                .validate_user(
                    &Email::parse("test@test.com").unwrap(),
                    &Password::parse("password123").unwrap()
                )
                .await,
            Ok(())
        );
    }
}
