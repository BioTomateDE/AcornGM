use std::path::PathBuf;

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

pub fn get_default_profile_path() -> Result<String, String> {
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
