use super::token::Token;
use super::utils::{username_to_config, username_to_dir};
use crate::lichess_api::{GamesConfiguration, Lichess};
use crate::Error;
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct User {
    username: String,
    token: Token,
    pub ultra_bullet: bool,
    pub bullet: bool,
    pub blitz: bool,
    pub rapid: bool,
    pub classical: bool,
    pub correspondence: bool,
    timestamp: i64,
}

impl User {
    pub(super) fn new(username: &str, token: &str, password: &str) -> Result<User, Error> {
        let t = Token::new(token, password)?;
        let user_dir = username_to_dir(username);
        if user_dir.is_dir() {
            return Err(Error::UserExists);
        }
        create_dir_all(user_dir)?;
        let user = User {
            username: username.to_string(),
            token: t,
            ultra_bullet: false,
            bullet: false,
            blitz: false,
            rapid: true,
            classical: true,
            correspondence: true,
            timestamp: 0,
        };
        user.save()?;
        Ok(user)
    }

    fn save(&self) -> Result<(), Error> {
        if !self.token.is_locked() {
            return Err(Error::SavingUnlockedConfiguration);
        }
        let s = serde_json::to_string(&self)?;
        fs::write(username_to_config(&self.username), &s)?;
        Ok(())
    }
    pub fn update(&self) -> Result<(), Error> {
        let locked_user = User::load(&self.username)?;
        let mut savable_user = self.clone();
        savable_user.token = locked_user.token;
        savable_user.save()
    }
    pub fn load(username: &str) -> Result<User, Error> {
        let s = fs::read(username_to_config(username))?;
        Ok(serde_json::from_slice(&s)?)
    }
    pub fn unlock(&mut self, password: &str) -> Result<(), Error> {
        self.token.unlock(password)
    }
    pub fn is_locked(&self) -> bool {
        self.token.is_locked()
    }
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
    pub async fn fetch(&mut self) -> Result<String, Error> {
        let mut lichess = Lichess::new(self.token.as_str()?)?;
        let mut c: GamesConfiguration = Default::default();
        c.since = self.timestamp * 1000;
        c.rated = true;
        let mut prefs = Vec::new();
        if self.ultra_bullet {
            prefs.push("ultraBullet".to_owned());
        }
        if self.bullet {
            prefs.push("bullet".to_owned());
        }
        if self.blitz {
            prefs.push("blitz".to_owned());
        }
        if self.rapid {
            prefs.push("rapid".to_owned());
        }
        if self.classical {
            prefs.push("classical".to_owned());
        }
        if self.correspondence {
            prefs.push("correspondence".to_owned());
        }
        c.perfType = prefs.join(",");
        self.timestamp = chrono::offset::Local::now().timestamp();
        Ok(lichess.games(&self.username, &c).await?)
    }
}
