use std::cmp::Ordering;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use reqwest::{Client, Response, Url};
use rfd::MessageDialogResult;
use serde::Deserialize;
use whoami::Platform;


const TEMP_FILE_NAME: &str = "updater_temp_exe";

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    assets: Vec<GitHubAsset>,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    state: String,
    browser_download_url: String,
}


fn parse_semver(raw_version: &str) -> Result<semver::Version, String> {
    semver::Version::parse(raw_version).map_err(|e| format!("Could not parse SemVer \"{}\": {e}", raw_version))
}

fn build_client() -> Result<Client, String> {
    reqwest::ClientBuilder::new()
        .user_agent("rust/acorngm/updater")
        .build()
        .map_err(|e| format!("Could not build reqwest Client: {e}"))
}

fn build_url(url_str: &str) -> Result<Url, String> {
    Url::from_str(url_str).map_err(|e| format!("Could not convert URL string \"{url_str}\" into reqwest URL: {e}"))
}


/// Checks for newer releases in GitHub/BioTomateDE/AcornGM.
/// Returns URL to download new binary for this platform.
pub async fn check_for_updates() -> Result<Option<String>, String> {
    let platform_keyword: &str = match whoami::platform() {
        Platform::Linux | Platform::Bsd | Platform::Illumos => "linux",
        Platform::Windows => "windows",
        Platform::MacOS => "macos",
        other => {
            log::warn!("Unknown/Unsupported Operating System: {other}; cannot use auto updater");
            return Ok(None)
        }
    };

    let self_version = semver::Version::parse(env!("CARGO_PKG_VERSION"))
        .map_err(|e| format!("Could not parse SemVer CARGO_PKG_VERSION: {e}"))?;

    let url: Url = build_url("https://api.github.com/repos/BioTomateDE/AcornGM/releases?per_page=9999&page=1")?;
    let client: Client = build_client()?;

    let response: Response = client.get(url)
        .header("Accept", "application/vnd.github+json")
        .send().await
        .map_err(|e| format!("Error while sending request to GitHub API to get releases: {e}"))?;

    // let text: String = response.text().await.map_err(|e| format!("Could not get text from GitHub API response: {e}"))?;
    let releases: Vec<GitHubRelease> = response.json().await.map_err(|e| format!("Could not get json from GitHub API response: {e}"))?;
    let mut latest_release: &GitHubRelease = releases.first().ok_or("GitHub API responded with an empty release list")?;
    let mut latest_release_ver = parse_semver(&latest_release.tag_name)?;

    for release in &releases {
        if release.draft || release.prerelease || !["main", "master"].contains(&&*release.target_commitish) {
            continue
        }
        let release_ver = parse_semver(&release.tag_name)?;
        if release_ver > latest_release_ver {
            latest_release = release;
            latest_release_ver = release_ver;
        }
    }

    log::info!("Latest GitHub release \"{}\" has version {} which is {} the current version {}",
        latest_release.name,
        latest_release_ver,
        match latest_release_ver.cmp(&self_version) {
            Ordering::Less => "older than",
            Ordering::Equal => "the same as",
            Ordering::Greater => "newer than",
        },
        self_version,
    );

    if self_version >= latest_release_ver {
        return Ok(None)   // no need to update; already latest version
    }

    for asset in &latest_release.assets {
        if asset.state != "uploaded" || !asset.name.to_lowercase().contains(platform_keyword) {
            continue
        }
        return Ok(Some(asset.browser_download_url.clone()))
    }

    let asset_names: String = latest_release.assets.iter().map(|i| i.name.clone()).collect::<Vec<String>>().join(", ");
    log::warn!("Release \"{}\" does not have any matching assets for platform keyword \"{}\": [{}] assets", latest_release.name, platform_keyword, asset_names);
    Ok(None)
}


pub async fn download_update_file(home_dir: PathBuf, asset_file_url: String) -> Result<(), String> {
    log::info!("Download update file {asset_file_url}");

    let temp_file_path: PathBuf = home_dir.join("temp").join(TEMP_FILE_NAME);
    if temp_file_path.exists() {
        let message = "Temporary update file already exists!\nPlease make sure there is no other AcornGM instance running while updating.";
        const CHOICE_QUIT: &str = "Quit";
        const CHOICE_IGNORE: &str = "Start & Ignore";
        const CHOICE_DELETE: &str = "Start & Delete";
        
        let dialogue_result: MessageDialogResult = rfd::MessageDialog::new()
            .set_title("AcornGM Updater conflict")
            .set_description(message)
            .set_buttons(rfd::MessageButtons::YesNoCancelCustom(CHOICE_DELETE.to_string(), CHOICE_IGNORE.to_string(), CHOICE_QUIT.to_string()))
            .set_level(rfd::MessageLevel::Error)
            .show();
        
        match dialogue_result {
            MessageDialogResult::Custom(string) if string == CHOICE_QUIT => std::process::exit(0),
            MessageDialogResult::Custom(string) if string == CHOICE_IGNORE => return Ok(()),
            MessageDialogResult::Custom(string) if string == CHOICE_DELETE => {
                cancel_update(&home_dir)?;
                return Ok(())
            },
            other => return Err(format!("(internal error) Unknown Message Dialogue Result \"{other}\"")),
        }
    }

    let url: Url = build_url(&asset_file_url)?;
    let client: Client = build_client()?;
    let response: Response = client.get(url)
        .send().await
        .map_err(|e| format!("Error while sending request to GitHub API to download asset file: {e}"))?;

    let mut file = File::create(&temp_file_path)
        .map_err(|e| format!("Failed to create temporary update file \"{}\": {e}", temp_file_path.display()))?;

    let content = response.bytes().await
        .map_err(|e| format!("Could not download asset file: {e}"))?;

    std::io::copy(&mut Cursor::new(content), &mut file)
        .map_err(|e| format!("Could not copy asset file contents: {e}"))?;

    Ok(())
}


pub fn install_update(home_dir: &Path) -> Result<(), String> {
    log::info!("Installing update...");
    let temp_file_path: PathBuf = home_dir.join("temp").join(TEMP_FILE_NAME);
    // TODO

    if let Err(e) = install_update_unix(&temp_file_path) {
        cancel_update(home_dir)?;
        return Err(e)
    };

    Ok(())
}

fn install_update_unix(temp_file_path: &Path) -> Result<(), String> {
    let cur_exe_path: PathBuf = std::env::current_exe()
        .map_err(|e| format!("Could not get path of current executable file: {e}"))?;
    // let cur_exe_extension: &str = match cur_exe_path.extension() {
    //     Some(os_str) => os_str.to_str().ok_or_else(|| format!("Could not convert file extension OsStr: {os_str:?}"))?,
    //     None => "",
    // };

    log::info!("Setting file permissions of new executable file...");
    let metadata = std::fs::metadata(&cur_exe_path)
        .map_err(|e| format!("Could not get file metadata of current executable file: {e}"))?;
    std::fs::set_permissions(temp_file_path, metadata.permissions())
        .map_err(|e| format!("Could not set file permissions of new executable file: {e}"))?;
    
    log::info!("Replacing the executable file...");
    std::fs::rename(temp_file_path, &cur_exe_path)
        .map_err(|e| format!("Could not replace executable file: {e}"))?;

    let mut cmd = std::process::Command::new(cur_exe_path);

    // preserve commandline arguments and environment variables
    cmd.args(std::env::args_os().skip(1));  // skip first argument (program name)
    if let Ok(value) = std::env::var("ACORNGM_HOME") {
        cmd.envs([("ACORNGM_HOME", value)]);    // might not be necessary if always inherited?
    }
    log::info!("Launching new executable: {cmd:?}");
    cmd.spawn().map_err(|e| format!("Could not restart executable: {e}"))?;

    log::info!("Quitting old instance");
    std::process::exit(0);
}


pub fn cancel_update(home_dir: &Path) -> Result<(), String> {
    // FIXME: better solution for this? deleting the file seems unnecessary
    log::info!("Update was cancelled.");
    let temp_file_path: PathBuf = home_dir.join("temp").join(TEMP_FILE_NAME);
    if temp_file_path.exists() {
        std::fs::remove_file(temp_file_path).map_err(|e| format!("Could not remove temporary update file: {e}"))?;
    }
    Ok(())
}

