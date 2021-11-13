use crate::pgn::PgnParser;
use crate::user_info::{users_list, User};
use crate::utils::spinner_bar;
use crate::Error;
use clap::ArgMatches;
use pgn_reader::BufferedReader;
use rpassword::prompt_password_stdout;
pub async fn fetch(args: &ArgMatches<'_>) -> Result<(), Error> {
    let name = args.value_of("username").unwrap().to_owned();
    if !users_list()
        .expect("Failed to retrieve the list of current users.")
        .contains(&name)
    {
        return Err(Error::UserDoesNotExist);
    }
    let mut user = User::load(&name)?;
    for i in 0..3 {
        let password = prompt_password_stdout("Password:")?;
        match user.unlock(&password) {
            Ok(_) => break,
            Err(_) => println!("Failing attempt {}/3. Retrying..", i + 1),
        }
    }
    if user.is_locked() {
        println!("Failed to unlock user profile, Exiting..");
        return Ok(());
    }
    let bar = spinner_bar();
    bar.set_message("Downloading games from https://lichess.org");
    let games_raw = user.fetch().await?;
    bar.set_message("Parsing Games...");
    let mut reader = BufferedReader::new_cursor(&games_raw);
    let mut parser = PgnParser::new();
    reader.read_all(&mut parser)?;
    let games = parser.games();
    bar.finish_with_message(format!("Done parsing {} games", games.len()));
    unimplemented!();
    user.update()
}
