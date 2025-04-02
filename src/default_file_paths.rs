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

pub fn get_default_profile_directory() -> Result<PathBuf, String> {
    let username: String = whoami::username();
    if username == "" {
        return Err("Username returned by whoami::username() is empty.".to_string());
    }

    let dir: String = match std::env::consts::OS {
        "windows" => format!("C:/Users/{username}/Documents"),
        "linux" => format!("/home/{username}/Documents"),
        // "macos" => Ok(format!("idk, i don't use macOS")),
        other => return Err(format!("Unknown or unsupported operating system \"{other}\".")),
    };

    let dir: PathBuf = PathBuf::from(dir);
    if !dir.is_dir() {
        return Err(format!("Default home directory doesn't exist or is not a directory: {}", dir.to_str().unwrap()));
    }

    Ok(dir.join("./AcornGM/"))
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


static FAIL_STR: &'static str = "Everything that could've gone wrong, did go wrong. Consider getting an operating system that people actually use.";

pub fn get_home_directory() -> PathBuf {
    
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
    let error_msg: String;
    println!("Environment Variable for Profiles Directory (ACORNGM_HOME) not set; using default directory.");
    match get_default_profile_directory() {
        Ok(path) => {
            return path;
        }
        Err(error) => error_msg = error,
    }

    // if failed, prompt profile dir
    show_msgbox("Set AcornGM Profiles Directory", &format!(
        "Failed to find AcornGM profile folder: \"{error_msg}\"\
        \nPlease enter it manually.\
        \n\nIf this is a reoccurring issue, the program cannot set the environment variable \
        correctly and your operating system might be unsupported (open a ticket on GitHub)."
    ));

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
                    show_msgbox("Invalid Directory", "Your specified directory doesn't exist!");
                    continue
                }
                return path
            },
        }
    }

}

fn get_env_var() -> Option<String> {
    match std::env::var("ACORNGM_HOME") {
        Ok(ok) => Some(ok),
        Err(_) => None
    }
}


pub fn show_msgbox(title: &str, message: &str) {
    println!("Showing MsgBox: {message}");

    let message_box = dialog::Message::new(message)
        .title(title)
        .show();

    match message_box {
        Ok(_) => {},
        Err(error) => println!("Failed to show message box: {error}")
    }
}

