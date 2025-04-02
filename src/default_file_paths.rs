use std::path::PathBuf;
use dialog::DialogBox;

pub fn get_default_image_prompt_path() -> Result<String, String> {
    let username: String = whoami::username();
    if username == "" {
        return Err("Username returned by whoami::username() is empty.".to_string());
    }

    match std::env::consts::OS {
        "windows" => Ok(format!("C:/Users/{username}/Pictures/")),
        "linux" => Ok(format!("/home/{username}/Pictures/")),
        // "macos" => Ok(format!("idk, i don't use macOS")),
        other => Err(format!("Unknown or unsupported operating system \"{other}\".")),
    }
}

pub fn get_default_profile_directory() -> Result<String, String> {
    let username: String = whoami::username();
    if username == "" {
        return Err("Username returned by whoami::username() is empty.".to_string());
    }

    match std::env::consts::OS {
        "windows" => Ok(format!("C:/Users/{username}/Documents/AcornGM/")),
        "linux" => Ok(format!("/home/{username}/Documents/AcornGM/")),
        // "macos" => Ok(format!("idk, i don't use macOS")),
        other => Err(format!("Unknown or unsupported operating system \"{other}\".")),
    }

    // dbg!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n", whoami::username(), whoami::arch(), whoami::desktop_env(),
    // whoami::devicename(), whoami::platform(), whoami::realname(), whoami::distro());
}

pub fn get_default_data_file_dir() -> Result<PathBuf, String> {
    let username: String = whoami::username();
    if username == "" {
        return Err("Username returned by whoami::username() is empty.".to_string());
    }

    let path_orig: PathBuf = match std::env::consts::OS {
        "windows" => PathBuf::from(&"C:/Program Files (x86)/Steam/steamapps/common/Undertale/"),
        "linux" => PathBuf::from(format!("/home/{username}/.steam/steam/steamapps/common/Undertale/assets/")),
        other => return Err(format!("Unknown or unsupported operating system \"{other}\".")),
    };

    let mut path: PathBuf = path_orig.clone();
    while !path.exists() {
        path = match path.parent() {
            Some(p) => p.to_path_buf(),
            None => return Err(format!("Path doesn't exist at all somehow: {:?}", path)),
        };
    }
    Ok(path)
}


pub fn get_home_directory() -> PathBuf {
    static FAIL_STR: &'static str = "Everything that could've gone wrong, did go wrong. Consider getting an operating system that people actually use.";
    
    // try to find path in environment variables
    match get_env_var() {
        Some(string) => {
            let path = PathBuf::from(string);
            if path.is_dir() {
                return path
            }
        },
        None => {}
    };
    
    // if not found, use default profile dir
    let mut error_msg: String = "".to_string();
    println!("Environment Variable for Profiles Directory (ACORNGM_HOME) doesn't exist; trying to create it...");
    match get_default_profile_directory() {
        Ok(string) => {
            let path = PathBuf::from(string.clone());
            if path.is_dir() {
                set_env_var(&string);
                return path
            }
            error_msg = format!("Default directory doesn't exist: {string}");
        }
        Err(error) => error_msg = error,
    }

    // if failed, prompt profile dir
    dialog::Message::new(
        &format!(
            "Failed to find AcornGM profile folder: \"{error_msg}\"\
            \nPlease enter it manually.\
            \n\nIf this is a reoccurring issue, the program cannot set the environment variable \
            correctly and your operating system might be unsupported (open a ticket on GitHub).")
    )
        .title("Set AcornGM Profiles Directory")
        .show()
        .expect(FAIL_STR);

    loop {
        let string: Option<String> = dialog::Input::new("Enter your preferred Profiles Directory: ")
            .title("Set AcornGM Profiles Directory")
            .show()
            .expect(FAIL_STR);

        match string {
            None => continue,
            Some(string) => {
                let path = PathBuf::from(string.clone());
                if !path.is_dir() {
                    dialog::Message::new("Your specified directory doesn't exist!")
                        .title("Invalid Directory")
                        .show()
                        .expect(FAIL_STR);
                    continue
                }

                // finally
                set_env_var(&string);
                return path
            },
        }
    }

}


fn set_env_var(value: &str) {
    unsafe {
        std::env::set_var("ACORNGM_HOME", value);
    }
}

fn get_env_var() -> Option<String> {
    match std::env::var("ACORNGM_HOME") {
        Ok(ok) => Some(ok),
        Err(_) => None
    }
}
