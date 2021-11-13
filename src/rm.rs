use crate::user_info::{remove_user, users_list};
use crate::utils::input;
use crate::Error;
use clap::ArgMatches;
pub fn rm(args: &ArgMatches) -> Result<(), Error> {
    let name = args.value_of("username").unwrap().to_owned();
    if !users_list()
        .expect("Failed to retrieve the list of current users.")
        .contains(&name)
    {
        return Err(Error::UserDoesNotExist);
    }
    let name2 = input("WARNING!!! YOU ARE ABOUT TO DELETE A PROFILE. please rewrite the profile's name again to confirm:")?;
    if name == name2 {
        remove_user(&name)?;
        println!("User removed!");
    } else {
        println!("Named didn't match. Doing nothing!");
    }
    Ok(())
}
