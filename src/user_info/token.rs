use crate::Error;
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use hkdf::Hkdf;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(super) enum Token {
    Locked {
        salt: [u8; 32],
        nonce: [u8; 12],
        data: Vec<u8>,
    },
    Unlocked {
        token: String,
    },
}

impl Token {
    pub fn new(token: &str, password: &str) -> Result<Token, Error> {
        let mut salt = [0u8; 32];
        thread_rng().fill_bytes(&mut salt);
        let hkdf = Hkdf::<Sha3_256>::new(Some(&salt), password.as_bytes());
        let mut okm = [0u8; 32];
        hkdf.expand(&[], &mut okm).unwrap();

        let mut nonce_buf = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_buf);
        let key = Key::from_slice(&okm);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(&nonce_buf);
        let ciphertext = cipher.encrypt(nonce, token.as_bytes())?;
        Ok(Token::Locked {
            nonce: nonce_buf,
            salt,
            data: ciphertext,
        })
    }

    fn decrypted(
        salt: &[u8; 32],
        nonce: &[u8; 12],
        data: &[u8],
        password: &str,
    ) -> Result<String, Error> {
        let hkdf = Hkdf::<Sha3_256>::new(Some(salt), password.as_bytes());
        let mut okm = [0u8; 32];
        hkdf.expand(&[], &mut okm).unwrap();

        let key = Key::from_slice(&okm);
        let cipher = ChaCha20Poly1305::new(key);
        let cipher_nonce = Nonce::from_slice(nonce);
        let plain_text = cipher.decrypt(cipher_nonce, data.as_ref())?;
        let token = String::from_utf8_lossy(&plain_text).into_owned();
        Ok(token)
    }
    pub fn unlock(&mut self, password: &str) -> Result<(), Error> {
        let token = match self {
            Token::Unlocked { .. } => return Ok(()),
            Token::Locked { salt, nonce, data } => Token::decrypted(salt, nonce, data, password)?,
        };
        *self = Token::Unlocked { token };
        Ok(())
    }

    pub fn is_locked(&self) -> bool {
        match self {
            Token::Locked { .. } => true,
            Token::Unlocked { .. } => false,
        }
    }
    pub fn as_str(&self) -> Result<&str, Error> {
        match self {
            Token::Locked { .. } => Err(Error::TokenMustBeUnlocked),
            Token::Unlocked { token } => Ok(token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    #[test]
    fn token_locking_test_() {
        let token_str = "123456789876543234567";
        let password = "123456";
        let mut token = Token::new(token_str, password).unwrap();
        assert!(token.is_locked());
        token.unlock(password).unwrap();
        match token {
            Token::Unlocked { token } => assert_eq!(token, token_str),
            _ => panic!("The token should have been unlocked"),
        }
    }
}
