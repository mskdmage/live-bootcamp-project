use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::domain::{User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {

        match self.users.entry(user.email.clone()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            },
        }
        
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {

        self.users
            .get(email)
            .cloned() // Cloning to comply with function signature
            .ok_or(UserStoreError::UserNotFound)

    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {

        let user = self.users
            .get(email)
            .ok_or(UserStoreError::UserNotFound)?;
        
        if password != user.password {
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
        let new_user = User::new("test@test.com", "password123", true);
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
        let new_user = User::new("test@test.com", "password123", true);
        let email = new_user.email.clone();

        store.add_user(new_user.clone()).await.unwrap();

        let fetched = store.get_user(&email).await;
        assert_eq!(fetched, Ok(new_user));

        assert_eq!(
            store.get_user("notfound@test.com").await,
            Err(UserStoreError::UserNotFound)
        );

    }

    #[tokio::test]
    async fn test_validate_user() {

        let mut store = HashmapUserStore::default();
        let new_user = User::new("test@test.com", "password123", true);

        store.add_user(new_user).await.unwrap();

        assert_eq!(
            store.validate_user("wrong@test.com", "password123").await,
            Err(UserStoreError::UserNotFound)
        );

        assert_eq!(
            store.validate_user("test@test.com", "wrongpassword").await,
            Err(UserStoreError::InvalidCredentials)
        );

        assert_eq!(
            store.validate_user("test@test.com", "password123").await,
            Ok(())
        );

    }
}
