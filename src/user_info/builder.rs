use super::User;
use crate::Error;

#[derive(Default)]
pub struct UserBuilder {
    username: Option<String>,
    //TODO replace with something secure,
    token: Option<String>,
    //TODO replace with something secure,
    password: Option<String>,
}

impl UserBuilder {
    pub fn new() -> UserBuilder {
        Default::default()
    }
    pub fn username(mut self, username: &str) -> UserBuilder {
        self.username = Some(username.to_string());
        self
    }
    pub fn token(mut self, token: &str) -> UserBuilder {
        self.token = Some(token.to_string());
        self
    }
    pub fn password(mut self, password: &str) -> UserBuilder {
        self.password = Some(password.to_string());
        self
    }
    pub fn build(self) -> Result<User, Error> {
        let username = self
            .username
            .map(Ok)
            .unwrap_or(Err(Error::UserNameNotProvided))?;
        match (self.token, self.password) {
            (Some(token), Some(password)) => Ok(User::new(&username, &token, &password)?),
            (Some(_), None) => Err(Error::TokenMustHavePassword),
            (None, Some(_)) => Err(Error::UnusedPassword),
            (None, None) => Err(Error::TokenRequired),
        }
    }
}
