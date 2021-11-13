use crate::consts::PROJ_DIR;
use crate::Error;
use std::fs;
use std::path::PathBuf;

pub(super) fn username_to_dir(username: &str) -> PathBuf {
    let path_dir = PROJ_DIR.config_dir();
    let mut user_dir = PathBuf::from(path_dir);
    user_dir.push(username);
    user_dir
}

pub(super) fn username_to_config(username: &str) -> PathBuf {
    let mut p = username_to_dir(username);
    p.push("config");
    p
}

pub fn users_list() -> Result<Vec<String>, Error> {
    Ok(fs::read_dir(PROJ_DIR.config_dir())?
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap().file_name().into_string().unwrap_or_default())
        .filter(|x| !x.is_empty())
        .collect())
}

pub fn remove_user(username: &str) -> Result<(), Error> {
    let path = username_to_dir(username);
    Ok(fs::remove_dir_all(path)?)
}
