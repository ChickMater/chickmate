use crate::Error;
use indicatif::{ProgressBar, ProgressStyle};
use rpassword::prompt_password_stdout;
use std::io::{self, Write};
pub fn input(prelude: &str) -> Result<String, Error> {
    let mut input = String::new();
    print!("{} ", prelude);
    io::stdout().flush()?;
    while input.trim().is_empty() {
        io::stdin().read_line(&mut input)?;
    }
    Ok(input.trim().to_owned())
}

pub fn input_yn(prelude: &str, default: bool) -> Result<bool, Error> {
    let mut input = String::new();
    print!("{} ", prelude);
    if default {
        print!("[Y/n] :");
    } else {
        print!("[N/y] :");
    }
    io::stdout().flush()?;
    loop {
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "" => return Ok(default),
            "y" => return Ok(true),
            "Y" => return Ok(true),
            "n" => return Ok(false),
            "N" => return Ok(false),
            _ => (),
        };
    }
}

pub fn get_password_twice() -> String {
    loop {
        let password1 =
            prompt_password_stdout("Please choose a new password for to protect your token:")
                .expect("Failed to read password");
        let password2 = prompt_password_stdout("Please enter the password again:")
            .expect("Failed to read password");
        if password1 == password2 {
            return password1;
        } else {
            println!("Password does not match!")
        }
    }
}

pub fn spinner_bar() -> ProgressBar {
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(120);
    bar.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ])
            .template("{spinner:.blue} {msg}"),
    );
    bar
}
