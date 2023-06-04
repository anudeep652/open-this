use std::env::{self, args};

use std::path::Path;
use std::process::exit;

use open_this::{read_dirs, search_all_dirs, set_user, HOME_DIR};

fn main() {
    let user = get_user().unwrap();
    set_user(user.as_str());

    let args = args().collect::<Vec<String>>();

    if Path::exists(Path::new(unsafe { HOME_DIR.as_str() })) {
        let files = read_dirs(unsafe { HOME_DIR.as_str() });

        search_all_dirs(files, &args)
    }
}

fn get_user() -> Option<String> {
    let username = if cfg!(target_os = "linux") {
        env::var("USER")
    } else if cfg!(target_os = "windows") {
        env::var("USERNAME")
    } else if cfg!(target_os = "macos") || cfg!(target_os = "macos") {
        env::var("LOGNAME")
    } else {
        Err(std::env::VarError::NotPresent)
    };

    match username {
        Ok(name) => {
            // println!("Current username: {}", name);
            Some(name)
        }
        Err(_) => {
            eprintln!("Failed to retrieve username");
            exit(1);
        }
    }
}
