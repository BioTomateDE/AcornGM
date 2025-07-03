use std::cmp::Ordering;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use reqwest::{Client, Response, Url};
use rfd::MessageDialogResult;
use serde::Deserialize;
use whoami::Platform;


const TEMP_EXECUTABLE_FILENAME: &str = "updater_temp_exe";
#[cfg(unix)]
const TEMP_SHELL_SCRIPT_FILENAME: &str = "updater_temp_script.sh";
#[cfg(windows)]
const TEMP_SHELL_SCRIPT_FILENAME: &str = "updater_temp_script.ps1";
#[cfg(all(not(unix), not(windows)))]
compiler_error!("Updater not support on platforms other than Unix and Windows.");

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


pub fn check_if_updated(home_dir: &Path) -> Result<bool, String> {
    let shell_script_path: PathBuf = home_dir.join("temp").join(TEMP_SHELL_SCRIPT_FILENAME);
    if shell_script_path.exists() {
        std::fs::remove_file(shell_script_path).map_err(|e| format!("Could not delete temporary shell script: {e}"))?;
        return Ok(true)
    }
    Ok(false)
}


/// Checks for newer releases in GitHub/BioTomateDE/AcornGM.
/// Returns URL to download new binary for this platform.
pub async fn check_for_updates() -> Result<Option<String>, String> {
    // TODO remove temp shell updater script
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

    let temp_file_path: PathBuf = home_dir.join("temp").join(TEMP_EXECUTABLE_FILENAME);
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
            MessageDialogResult::Custom(string) if string == CHOICE_DELETE => cancel_update(&home_dir)?,
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
    let temp_file_path: PathBuf = home_dir.join("temp").join(TEMP_EXECUTABLE_FILENAME);

    #[cfg(unix)] {
        if let Err(e) = install_update_unix(&temp_file_path) {
            cancel_update(home_dir)?;
            return Err(e)
        };
    }
    #[cfg(windows)] {
        if let Err(e) = install_update_windows(&temp_file_path) {
            cancel_update(home_dir)?;
            return Err(e)
        };
    }
    #[cfg(all(not(unix), not(windows)))] {
        return Err(format!("Unsupported platform {}; cannot update", whoami::platform()));
    }

    Ok(())
}

#[cfg(unix)]
fn install_update_unix(temp_file_path: &Path) -> Result<(), String> {
    let cur_exe_path: PathBuf = std::env::current_exe()
        .map_err(|e| format!("Could not get path of current executable file: {e}"))?;

    log::info!("Setting file permissions of new executable file...");
    let metadata = std::fs::metadata(&cur_exe_path)
        .map_err(|e| format!("Could not get file metadata of current executable file: {e}"))?;
    std::fs::set_permissions(temp_file_path, metadata.permissions())
        .map_err(|e| format!("Could not set file permissions of new executable file: {e}"))?;

    log::info!("Replacing the executable file...");
    std::fs::rename(temp_file_path, &cur_exe_path)
        .map_err(|e| format!("Could not replace executable file: {e}"))?;

    let shell_script_path: PathBuf = temp_file_path.parent().ok_or("Temporary exe file does not have a parent")?.join(TEMP_SHELL_SCRIPT_FILENAME);
    let acorn_home: String = std::env::var("ACORNGM_HOME")
        .map(|var| format!("export ACORNGM_HOME={}", var))
        .unwrap_or_default();

    let script_contents = format!(r#"
        #!/usr/bin/env bash
        {}
        nohup '{}' &disown
        "#, acorn_home, cur_exe_path.display(),
    );
    std::fs::write(&shell_script_path, script_contents)
        .map_err(|e| format!("Could not write temporary shell script \"{}\": {e}", shell_script_path.display()))?;

    let mut cmd = std::process::Command::new("bash");
    cmd.arg(shell_script_path);
    log::info!("Launching new executable: {cmd:?}");
    cmd.spawn().map_err(|e| format!("Could not spawn temporary shell script to restart executable: {e}"))?;

    std::thread::sleep(Duration::from_millis(300));   // wait for the shell script to launch the new exe
    log::info!("Quitting old instance");
    std::process::exit(0);
}


fn install_update_windows(temp_file_path: &Path) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let cur_exe_path: PathBuf = std::env::current_exe()
        .map_err(|e| format!("Could not get path of current executable file: {e}"))?;

    let shell_script_path: PathBuf = temp_file_path.parent().ok_or("Temporary exe file does not have a parent")?.join(TEMP_SHELL_SCRIPT_FILENAME);
    let acorn_home: String = std::env::var("ACORNGM_HOME")
        .map(|var| format!("$env:ACORNGM_HOME = \"{}\"", var))
        .unwrap_or_default();

    let script_contents = format!(r#"
        [reflection.assembly]::LoadWithPartialName('System.Windows.Forms') | Out-Null;
        {0}
        $i = 0
        While (Get-Process -ProcessName AcornGM -ErrorAction SilentlyContinue) {{
            Start-Sleep 0.1
            $i += 1
            if ($i -gt 30) {{
                [windows.forms.messagebox]::Show(
                    'Could not update AcornGM executable file because the program is still running after 3 seconds!',
                    'AcornGM Updater Script',
                    0,
                    'Error'
                )
                Exit
            }}
        }}

        Move-Item -Path '{1}' -destination '{2}' -Force
        Start-Process '{2}'
        [windows.forms.messagebox]::Show(
            'AcornGM updated successfully!',
            'AcornGM Updater Script',
            0,
            'Information'
        )
        "#, acorn_home, temp_file_path.display(), cur_exe_path.display(),
    );

    std::fs::write(&shell_script_path, script_contents)
        .map_err(|e| format!("Could not write temporary PowerShell script \"{}\": {e}", shell_script_path.display()))?;

    log::info!("Launching PowerShell script");
    std::process::Command::new("powershell.exe")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-WindowStyle")
        .arg("Hidden")
        .arg(format!("powershell '{}'", shell_script_path.display()))
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("Failed to execute updater PowerShell script: {e}"))?;

    log::info!("Quitting old instance");
    std::process::exit(0);
}


pub fn cancel_update(home_dir: &Path) -> Result<(), String> {
    // FIXME: better solution for this? deleting the file seems unnecessary
    log::info!("Update was cancelled.");
    let temp_file_path: PathBuf = home_dir.join("temp").join(TEMP_EXECUTABLE_FILENAME);
    if temp_file_path.exists() {
        std::fs::remove_file(temp_file_path).map_err(|e| format!("Could not remove temporary update file: {e}"))?;
    }
    Ok(())
}

