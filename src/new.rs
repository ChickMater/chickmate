use crate::user_info::UserBuilder;
use crate::utils::*;
use crate::Error;

pub fn new_wizzard() -> Result<(), Error> {
    let username = input("Please enter your lichess username:")?;
    let token = input("Please enter your lichess API token (get one here https://lichess.org/account/oauth/token):")?;
    let password = get_password_twice();
    let mut user = UserBuilder::new()
        .username(&username)
        .token(&token)
        .password(&password)
        .build()
        .unwrap();

    user.ultra_bullet = input_yn("Analyse ultra-bullet games:", user.ultra_bullet)?;
    user.bullet = input_yn("Analyse bullet games:", user.bullet)?;
    user.blitz = input_yn("Analyse blitz games:", user.blitz)?;
    user.rapid = input_yn("Analyse rapid games:", user.rapid)?;
    user.classical = input_yn("Analyse classical games:", user.classical)?;
    user.correspondence = input_yn("Analyse correspondence games:", user.correspondence)?;
    user.update()
}
