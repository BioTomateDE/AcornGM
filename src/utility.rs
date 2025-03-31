use std::env;

pub fn get_default_profile_path() -> Result<String, String> {
    let username: String = whoami::username();
    if username == "" {
        return Err("Username returned by whoami::username() is empty.".to_string());
    }

    match env::consts::OS {
        "windows" => Ok(format!("C:/Users/{username}/Documents/GMAcorn/")),
        "linux" => Ok(format!("/home/{username}/Documents/GMAcorn/")),
        // "macos" => Ok(format!("idk, i don't use macOS")),
        other => Err(format!("Unknown or unsupported operating system \"{other}\".")),
    }

    // dbg!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n", whoami::username(), whoami::arch(), whoami::desktop_env(),
    // whoami::devicename(), whoami::platform(), whoami::realname(), whoami::distro());
}



// fn view_profiles(profiles: &[Profile]) -> Element<Message> {
//     container(
//         column(
//             profiles.iter().map(|i| i.view())
//         )
//     ).into()
// }

