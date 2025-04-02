
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


pub fn get_current_working_directory() -> Option<String> {
    match std::env::current_dir() {
        Ok(path) => match path.to_str() {
            Some(string) => Some(string.to_string()),
            None => {
                println!("[WARN]  Could not parse string of current working directory");
                None
            }
        },
        Err(error ) => {
            println!("[WARN]  Could not get current working directory: {error}");
            None
        }
    }
}


fn try_get_default_icon_image() -> Result<image::DynamicImage, ()> {
    let cwd = match get_current_working_directory() {
        Some(cwd) => cwd,
        None => return Err(()),
    };
    let path = std::path::Path::new(&cwd).join("./resources/default_profile_icon.png");

    let img = match image::open(path) {
        Ok(raw) => raw,
        Err(error) => {
            println!("[WARN @ utility::try_get_default_icon_image]  Failed to read default icon image: {error}");
            return Err(())
        }
    };

    Ok(img)
}

pub fn get_default_icon_image() -> image::DynamicImage {
    try_get_default_icon_image().unwrap_or_else(|_| image::DynamicImage::ImageRgba8(image::RgbaImage::new(256,256)))
}


pub fn img_to_iced(img: &image::DynamicImage) -> iced::widget::image::Image {
    let mut buf = std::io::Cursor::new(Vec::new());

    // Encode the image to PNG format in memory
    img.write_to(&mut buf, image::ImageOutputFormat::Png).unwrap();

    // Create an Iced `Handle` from memory bytes
    let handle = iced::widget::image::Handle::from_bytes(buf.into_inner());
    iced::widget::image::Image::new(handle)
}


