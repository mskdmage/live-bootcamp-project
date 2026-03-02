use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError, Token};

#[derive(Default)]
pub struct HashSetBannedTokenStore {
    banned_tokens: HashSet<Token>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    async fn add_token(&mut self, token: &Token) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token.clone());
        Ok(())
    }

    async fn is_token_banned(&self, token: &Token) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashSetBannedTokenStore::default();
        let token = Token::parse("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0QGV4YW1wbGUuY29tIiwiZXhwIjoxNjAwMDAwMDAwfQ.signature").unwrap();
        assert_eq!(store.add_token(&token).await, Ok(()));
    }

    #[tokio::test]
    async fn test_is_token_banned() {
        let mut store = HashSetBannedTokenStore::default();
        let token = Token::parse("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0QGV4YW1wbGUuY29tIiwiZXhwIjoxNjAwMDAwMDAwfQ.signature").unwrap();
        assert_eq!(store.is_token_banned(&token).await, Ok(false));
        assert_eq!(store.add_token(&token).await, Ok(()));
        assert_eq!(store.is_token_banned(&token).await, Ok(true));
    }

    #[tokio::test]
    async fn test_is_not_banned() {
        let store = HashSetBannedTokenStore::default();
        let token = Token::parse("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0QGV4YW1wbGUuY29tIiwiZXhwIjoxNjAwMDAwMDAwfQ.signature").unwrap();
        assert_eq!(store.is_token_banned(&token).await, Ok(false));
    }
}
