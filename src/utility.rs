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


