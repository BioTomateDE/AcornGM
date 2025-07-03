use std::path::{Path, PathBuf};

fn get_username() -> Result<String, String> {
    whoami::fallible::username().map_err(|e| format!("Could not get username: {e}"))
}

pub fn get_default_image_prompt_path() -> Result<PathBuf, String> {
    let username: String = get_username()?;

    match std::env::consts::OS {
        "windows" => Ok(PathBuf::from(format!("C:/Users/{username}/Pictures/"))),
        "linux" => Ok(PathBuf::from(format!("/home/{username}/Pictures/"))),
        // add other supported operating systems here
        other => Err(format!("Unsupported operating system \"{other}\".")),
    }
}

pub fn get_default_home_directory() -> Result<PathBuf, String> {
    let username: String = get_username()?;

    let dir: String = match std::env::consts::OS {
        "windows" => format!("C:/Users/{username}/"),
        "linux" => format!("/home/{username}/"),
        // add other supported operating systems here
        other => return Err(format!("Unsupported operating system \"{other}\".")),
    };

    let dir: PathBuf = PathBuf::from(dir);
    if !dir.is_dir() {
        return Err(format!("Default home directory doesn't exist or is not a directory: {}", dir.display()));
    }

    Ok(dir.join(".acorngm/"))
}

pub fn get_default_data_file_dir() -> Result<PathBuf, String> {
    let username: String = get_username()?;

    for steam_game in ["DELTARUNE/", "Undertale/assets/", "DeltaruneDEMO/", ""] {
        let paths_to_try: &[String] = match std::env::consts::OS {
            "windows" => &[
                format!("C:/Program Files (x86)/Steam/steamapps/common/{steam_game}"),
                format!("C:/Program Files/Steam/steamapps/common/{steam_game}"),
            ],
            "linux" => &[
                format!("/home/{username}/.steam/steam/steamapps/common/{steam_game}"),
                format!("/home/{username}/.var/app/com.valvesoftware.Steam/.steam/steam/steamapps/common/{steam_game}"),
                format!("/home/{username}/.local/share/Steam/steamapps/common/{steam_game}"),
                format!("/usr/share/steam/steamapps/common/{steam_game}"),
            ],
            // add other supported operating system here
            other => return Err(format!("Unsupported operating system \"{other}\"")),
        };
        for path in paths_to_try {
            let path = PathBuf::from(path);
            if path.is_dir() {
                return Ok(path)
            }
        }
    }
    
    Err("Could not find any steam location".to_string())
}


pub fn get_home_directory() -> PathBuf {
    // Prioritize environment variable
    if let Ok(string) = std::env::var("ACORNGM_HOME") {
        let path: &Path = Path::new(&string);
        if path.is_dir() {
            return path.to_path_buf()
        } else {
            log::warn!("Invalid home directory path specified by environment variable (ACORNGM_HOME): {string}");
        }
    };
    
    // If not found, use default profile dir
    log::debug!("Environment Variable for AcornGM home directory (ACORNGM_HOME) not set; trying to use default directory.");
    
    get_default_home_directory().unwrap_or_else(|error| {
        // Failed to get home directory; show error message and exit
        let message: String = format!(
            "Failed to find default AcornGM home folder: \"{error}\"\n\
            Please open an Issue on GitHub regarding this so your operating system can get native support.\n\
            To fix this error, set the environment variable ACORNGM_HOME to your desired folder path."
        );
        
        rfd::MessageDialog::new()
            .set_title("Fatal AcornGM Error")
            .set_description(message)
            .set_buttons(rfd::MessageButtons::Ok)
            .set_level(rfd::MessageLevel::Error)
            .show();
        
        std::process::exit(1);
    })
}


/// tries to check if this is your first time launching the program to prevent "home dir not found" errors.
pub fn check_if_first_launch(home_dir: &PathBuf) -> bool {
    !home_dir.is_dir() && home_dir.parent().map_or(true, |parent| parent.is_dir())
}

