use iced::Command;
use rfd::MessageDialogResult;
use crate::{Msg, MyApp, SceneType};
use crate::updater::{cancel_update, download_update_file, install_update};


#[derive(Debug, Clone)]
pub enum MsgGlobal {
    CheckedForUpdate(Result<Option<String>, String>),
    DownloadedUpdateFile(Result<(), String>),
    PromptedUpdate(MessageDialogResult),
}

impl MyApp {
    pub(crate) fn handle_global_messages(&mut self, message: MsgGlobal) -> Result<Command<Msg>, String> {
        match message {
            MsgGlobal::CheckedForUpdate(result) => if let Some(asset_file_url) = result? {
                if let SceneType::HomePage(ref mut scene) = self.active_scene {
                    scene.update_status_text = "Updating app...";
                }
                let future = download_update_file(self.home_dir.clone(), asset_file_url);
                return Ok(Command::perform(future, MsgGlobal::DownloadedUpdateFile).map(Msg::Global))
            }
            
            MsgGlobal::DownloadedUpdateFile(result) => {
                if let Err(e) = result {
                    if let SceneType::HomePage(ref mut scene) = self.active_scene {
                        scene.update_status_text = "";
                    }
                    return Err(e)
                };
                let future = rfd::AsyncMessageDialog::new()
                    .set_title("AcornGM Updater")
                    .set_description("AcornGM will now update")
                    .set_buttons(rfd::MessageButtons::OkCancelCustom("Install".to_string(), "Kys".to_string()))
                    .set_level(rfd::MessageLevel::Info)
                    .show();
                return Ok(Command::perform(future, MsgGlobal::PromptedUpdate).map(Msg::Global))
            }
            
            MsgGlobal::PromptedUpdate(dialogue_result) => {
                let should_update: bool = match dialogue_result {
                    MessageDialogResult::No => false,
                    MessageDialogResult::Cancel => false,
                    MessageDialogResult::Custom(string) if string == "Kys" => false,
                    MessageDialogResult::Yes => true,
                    MessageDialogResult::Ok => true,
                    MessageDialogResult::Custom(string) if string == "Install" => true,
                    MessageDialogResult::Custom(other) => return Err(format!("(internal error) Unknown Message Dialogue Result \"{other}\"")),
                };
                if should_update {
                    install_update(&self.home_dir)?;
                } else {
                    cancel_update(&self.home_dir)?;
                }
                if let SceneType::HomePage(ref mut scene) = self.active_scene {
                    scene.update_status_text = "";
                }
            },
        }

        Ok(Command::none())
    }
}

