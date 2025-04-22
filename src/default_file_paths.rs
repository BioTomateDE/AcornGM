use std::path::{Path, PathBuf};
use std::sync::Arc;
use biologischer_log::CustomLogger;
use log::{error, info, warn};
use crate::utility::show_error_dialogue;

pub fn get_default_image_prompt_path() -> Result<PathBuf, String> {
    let username: String = whoami::fallible::username()
        .map_err(|e| format!("Could not get username: {e}."))?;

    match std::env::consts::OS {
        "windows" => Ok(PathBuf::from(format!("C:/Users/{username}/Pictures/"))),
        "linux" => Ok(PathBuf::from(format!("/home/{username}/Pictures/"))),
        // add other supported operating systems here
        other => Err(format!("Unsupported operating system \"{other}\".")),
    }
}

pub fn get_default_home_directory() -> Result<PathBuf, String> {
    let username: String = whoami::fallible::username()
        .map_err(|e| format!("Could not get username: {e}."))?;

    let dir: String = match std::env::consts::OS {
        "windows" => format!("C:/Users/{username}/Documents"),
        "linux" => format!("/home/{username}/Documents"),
        // add other supported operating systems here
        other => return Err(format!("Unsupported operating system \"{other}\".")),
    };

    let dir: PathBuf = PathBuf::from(dir);
    if !dir.is_dir() {
        return Err(format!("Default home directory doesn't exist or is not a directory: {}", dir.display()));
    }

    Ok(dir.join("./AcornGM/"))
}

pub fn get_default_data_file_dir() -> Result<PathBuf, String> {
    let username: String = whoami::fallible::username()
        .map_err(|e| format!("Could not get username: {e}."))?;

    let path_orig: PathBuf = match std::env::consts::OS {
        "windows" => PathBuf::from(&"C:/Program Files (x86)/Steam/steamapps/common/Undertale/"),
        "linux" => PathBuf::from(format!("/home/{username}/.steam/steam/steamapps/common/Undertale/assets/")),
        // add other supported operating system here
        other => return Err(format!("Unsupported operating system \"{other}\".")),
    };

    // (Potentially) traverse path hierarchy upwards until the directory exists
    let mut path: PathBuf = path_orig.clone();
    while !path.exists() {
        path = match path.parent() {
            Some(p) => p.to_path_buf(),
            None => return Err(format!("Path doesn't exist at all somehow: {}", path.display())),
        };
    }
    Ok(path)
}


pub fn get_home_directory(logger: Arc<CustomLogger>) -> PathBuf {
    // Prioritize environment variable
    if let Ok(string) = std::env::var("ACORNGM_HOME") {
        let path: &Path = Path::new(&string);
        if path.is_dir() {
            return path.to_path_buf()
        } else {
            warn!("Invalid home directory path specified by environment variable (ACORNGM_HOME): {string}");
        }
    };
    
    // If not found, use default profile dir
    let error_msg: String;
    info!("Environment Variable for AcornGM home directory (ACORNGM_HOME) not set; trying to use default directory.");
    match get_default_home_directory() {
        Ok(path) => {
            return path;
        }
        Err(error) => error_msg = error,
    }

    // Failed; show error message and exit
    show_error_dialogue("Error getting AcornGM home directory", &format!(
        "Failed to find default AcornGM home folder: \"{error_msg}\"\n\
        Please open an Issue on GitHub regarding this so your operating system can get native support.\n\
        To fix this error, set the environment variable ACORNGM_HOME to your desired folder path."
    ));
    logger.shutdown();
    std::process::exit(1);
}

