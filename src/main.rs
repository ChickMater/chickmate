#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

mod consts;
mod error;
mod fetch;
mod lichess_api;
mod ls;
mod new;
mod pgn;
mod rm;
mod user_info;
mod utils;

use clap::{App, AppSettings};
use error::Error;
use fetch::fetch;
use ls::ls;
use new::new_wizzard;
use rm::rm;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml)
        .version(crate_version!())
        .version_short("v")
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    match matches.subcommand() {
        ("new", _) => new_wizzard(),
        ("ls", _) => ls(),
        ("rm", Some(args)) => rm(args),
        ("fetch", Some(args)) => fetch(args).await,
        _ => unimplemented!(),
    }
}
