use crate::Error;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Perf {
    pub games: u64,
    pub rating: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub username: String,
    pub perfs: HashMap<String, Perf>,
}

pub struct Lichess {
    client: Client,
    endpoint: String,
}

#[derive(Serialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct GamesConfiguration {
    pub since: i64,
    pub rated: bool,
    pub perfType: String,
}

impl Lichess {
    pub fn new(token: &str) -> Result<Lichess, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Lichess {
            client,
            endpoint: "https://lichess.org".to_owned(),
        })
    }
    pub async fn account(&mut self) -> Result<Account, Error> {
        let url = format!("{}/api/account", &self.endpoint);
        let res = self.client.get(&url).send().await?;
        Ok(res.json::<Account>().await?)
    }
    pub async fn games(
        &mut self,
        username: &str,
        conf: &GamesConfiguration,
    ) -> Result<String, Error> {
        let url = format!("{}/api/games/user/{}", &self.endpoint, username);
        let res = self.client.get(&url).query(conf).send().await?;
        Ok(res.text().await?)
    }
}
