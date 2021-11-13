use crate::user_info::{users_list, User};
use crate::Error;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Offset, TimeZone};
use comfy_table::Table;
pub fn ls() -> Result<(), Error> {
    let usernames = users_list().unwrap();
    let mut table = Table::new();
    table.set_header(vec![
        "Username",
        "Last updated",
        "Ultra Bullet",
        "Bullet",
        "Blitz",
        "Rapid",
        "Classical",
        "Correspondance",
    ]);
    let tz_offset = Local.timestamp(0, 0).offset().fix().local_minus_utc();
    let offset = FixedOffset::east(tz_offset);
    for username in usernames {
        match User::load(&username) {
            Err(e) => eprintln!("Failed to parse {}'s configuration {:?}", username, e),
            Ok(user) => {
                table.add_row(vec![
                    username,
                    match user.timestamp() {
                        0 => "Never".to_owned(),
                        secs => {
                            let ndt = NaiveDateTime::from_timestamp(secs, 0);
                            let dt: DateTime<FixedOffset> = DateTime::from_utc(ndt, offset);
                            format!("{}", dt.format("%Y-%m-%d %H:%M:%S"))
                        }
                    },
                    format!("{}", user.ultra_bullet),
                    format!("{}", user.bullet),
                    format!("{}", user.blitz),
                    format!("{}", user.rapid),
                    format!("{}", user.classical),
                    format!("{}", user.correspondence),
                ]);
            }
        }
    }
    println!("{}", table);
    Ok(())
}
